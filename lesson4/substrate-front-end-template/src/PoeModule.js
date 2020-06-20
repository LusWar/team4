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
  const [note, setNote] = useState(0);
  const [ownerId, setOwnerId] = useState('');
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

  const getDocInfoFromAddr = async (ownerId) => {
    
    const res = await api.query.poeModule.notes(ownerId);
    
    let claims = [];

    for (let i = 0; i < res.length; i++) {
      claims.push({
        claim: blake2AsHex(res[i][0], 256),
        blockNumber: parseInt(res[i][1]),
        note: res[i][2],
        createTime: new Date(parseInt(res[i][3]))
      });
    }
    console.log(claims);
    setClaimList(JSON.stringify(claims));
  }

  return (
    <Grid.Column width={8}>
      <h1>POE Module</h1>
      <Form>
        <Form.Field>
          <Input 
            type='file'
            id='file'
            label='文件'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>
        <Form.Field>
        <Input 
          type='text'
          id='note'
          label='备注'
          maxLength='256'
          state='note'
          onChange={ (_, { value }) => setNote(value) }
        />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label="Create Claim With Note"
            setStatus={setStatus}
            type="SIGNED-TX"
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaimWithNote',
              inputParams: [digest, note],
              paramFields: [true]
            }} />
        </Form.Field>

        <div>{status}</div>
        <div>{`Claim Info, owner: ${owner}, blockNumber: ${blockNumber}, note: ${note}`}</div>
      </Form>

      <Form>
        <Form.Field>
          <Input
            type="string"
            id="ownerId"
            label="OwnerID"
            state="ownerId"
            maxLength="256"
            onChange={ (_, {value }) => setOwnerId(value) }
          />
        </Form.Field>

        <Form.Field>
          <button onClick={() => getDocInfoFromAddr(ownerId)}>
            查询
          </button>
        </Form.Field>

        <div>{claimList}</div>
      </Form>
        
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.notes? <Main {...props} /> : null);
}