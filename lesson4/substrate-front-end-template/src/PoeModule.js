import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, TextArea, Label } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [note, setNote] = useState('');
  const [dest, setDest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [showingNotification, setShowingNotification] = useState(false);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[2].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    const fileReader = new FileReader();
    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');
      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  };

  const onDestChange = (_, data) => {
    setDest(data.value);
  };

  const MAX_NOTE_LENGTH = 256;

  const onNoteChange = (_, data) => {
    if (data.value && data.value.length > MAX_NOTE_LENGTH) {
      data.value = data.value.substring(0, MAX_NOTE_LENGTH);
    }
    setNote(data.value);
  };

  const setExtrinsicStatus = (data) => {
    console.log(data);
    console.log(data.indexOf('Finalized'));
    if (data.indexOf('Finalized') !== -1) {
      setShowingNotification(true);
      setTimeout(() => setShowingNotification(false), 5000);
    }
    setStatus(data);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
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
          <Label>Note</Label>
          <TextArea
            type='text'
            placeholder='Some note (max 256 chars)'
            state='note'
            maxLength={256}
            onChange={onNoteChange}
          />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='To'
            type='text'
            placeholder='address'
            state='dest'
            onChange={onDestChange}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setExtrinsicStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, note],
              paramFields: [true, true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, dest],
              paramFields: [true, true]
            }}
          />
        </Form.Field>
      </Form>
      {showingNotification && <SuccessNotification digest={digest} note={note}/>}
      <div style={{ marginTop: 10 }}>{status}</div>
      <div style={{ fontSize: 12, color: 'orange' }}>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
    </Grid.Column>
  );
}

const SuccessNotification = (props) => {
  const { digest, note } = props;
  const notificationStyle = {
    marginTop: 10,
    border: '1px solid green',
    backgroundColor: 'lightgreen',
    color: 'darkgreen',
    borderRadius: 5,
    padding: 10
  };
  return (
    <div style={notificationStyle}>
      You have successfully claimed file with hash {digest}, and note <strong>"{note}"</strong>.
    </div>
  );
};

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
