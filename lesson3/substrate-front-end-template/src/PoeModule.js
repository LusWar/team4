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
  const [amount, setAmount] = useState(0);


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
            type='string'
            id='receiver'
            label='Claim Receiver'
        
            state='receiver'
            onChange={ (_, { value }) => setReceiver(value) }
          />
          </Form.Field>
          <Form.Field>
          <Input
            fluid
            type='number'
            id='amount'
            label='Amount'
            state='amount'
            onChange={ (_, { value }) => setAmount(value) }
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
              inputParams: [digest],
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

            <TxButton
              accountPair={accountPair}
              label="Attach Claim Price"
              setStatus={setStatus}
              type="SIGNED-TX"
              attrs={{
                palletRpc: 'poeModule',
                callable: 'attachClaimPrice',
                inputParams: [digest, amount],
                paramFields: [true]
            }} />

            <TxButton
              accountPair={accountPair}
              label="Purchase Claim"
              setStatus={setStatus}
              type="SIGNED-TX"
              attrs={{
                palletRpc: 'poeModule',
                callable: 'purchaseClaim',
                inputParams: [digest, amount],
                paramFields: [true]
            }} />

        </Form.Field>

        <div>{status}</div>
        <div>{`Claim Info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
      </Form>
        
    </Grid.Column>
  );
}

export default function TemplateModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
