import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [receiver, setReceiver] = useState('');
  const [memo, setMemo] = useState(0);
  const [claimOwnerId, setClaimOwnerId] = useState('');
  const [claimList, setClaimList] = useState([]);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());      
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    let fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    }
    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  }

  const binToString = (array) => {
    let res = "";
    for (let i = 0; i < array.length; i++) {
      res += String.fromCharCode(parseInt(array[i]));
    }

    return res;
  }

  const getDocInfoFromAddr = async (claimOwnerId) => {
    
    const res = await api.query.poeModule.proofsOf(claimOwnerId);
    
    let claims = [];

    for (let i = 0; i < res.length; i++) {
      claims.push({
        claim: blake2AsHex(res[i][0], 256),
        blockNumber: parseInt(res[i][1]),
        memo: res[i][2],
        createTime: new Date(parseInt(res[i][3]))
      })
    }
    console.log(claims);
    setClaimList(JSON.stringify(claims));

  }

  return (
    <Grid.Column width={8}>
      <h1>Proof of existence Module</h1>
      <Form>
        <Form.Field>
          <Input 
            type='file'
            id='file'
            label='Your File'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>
        <Form.Field>
        <Input 
          type='text'
          id='memo'
          label='Claim Description'
          maxLength='256'
          state='memo'
          onChange={ (_, { value }) => setMemo(value) }
        />
        </Form.Field>
        <Form.Field>
        <Input 
          type='string'
          id='receiver'
          label='Claim Receiver'
      
          state='receiver'
          onChange={ (_, { value }) => setReceiver(value) }
        />
        </Form.Field>
          
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label="Create Claim"
            setStatus={setStatus}
            type="SIGNED-TX"
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, memo],
              paramFields: [true]
            }} />
            
          <TxButton
            accountPair={accountPair}
            label="Revoke Claim"
            setStatus={setStatus}
            type="SIGNED-TX"
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }} />

          <TxButton
            accountPair={accountPair}
            label="Transfer Claim"
            setStatus={setStatus}
            type="SIGNED-TX"
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, receiver],
              paramFields: [true]
            }} />


        </Form.Field>

        <div>{status}</div>
        <div>{`Claim Info, owner: ${owner}, blockNumber: ${blockNumber}, description: ${memo}`}</div>
      </Form>

      <Form>
        <Form.Field>
          <Input
            type="string"
            id="claimOwnerId"
            label="Claim Owner Id"
            state="claimOwnerId"
            maxLength="256"
            onChange={ (_, {value }) => setClaimOwnerId(value) }
          />
        </Form.Field>

        <Form.Field>
          <button onClick={() => getDocInfoFromAddr(claimOwnerId)}>
            Query Document Info
          </button>
        </Form.Field>

        <div>{claimList}</div>
      </Form>
        
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs 
    ? <Main {...props} /> : null);
}
