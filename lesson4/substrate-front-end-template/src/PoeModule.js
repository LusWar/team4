import React, { useEffect, useState } from 'react';
import { Table, Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [memo, setMemo] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [status, setStatus] = useState('');
  const [records, setRecords] = useState([]);
  const [id, setId] = useState('');

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
        setOwner(result[0].toString());
        setBlockNumber(result[1].toNumber());
        setMemo(result[2].toString());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const getRecords = async (id) => {
    try {
      const data = await api.query.poeModule.proofs.entries();
      let records_data = [];
      data.forEach( ([key, value]) => {
          if (id === value[0].toString()) {
            key = key.args.map((k) => k.toString())[0];
            records_data.push({key: key, address: value[0].toString(), block: value[1].toNumber(), memo: value[2].toString()});
          }
      });
      setRecords(records_data);
    } catch (e) {
      console.error(e);
    }
  };

  let fileReader;

  // Takes our file, and creates a digest using the Blake2 256 hash function.
  const bufferToDigest = () => {
    // Turns the file content to a hexadecimal representation.
    const content = Array.from(new Uint8Array(fileReader.result))
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');

    const hash = blake2AsHex(content, 256);
    setDigest(hash);
  };

  // Callback function for when a new file is selected.
  const handleFileChosen = (file) => {
    fileReader = new FileReader();
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  };

  return (
    <Grid.Column>
      <h1>Records</h1>
      <Form>
      <Input
        type='text'
        id='id'
        label='id'
        onChange = { (e) => setId(e.target.value)}
      />
      <div>
      <button onClick={() => getRecords(id)}>
          Query Claim
      </button>
      </div>
      <Table celled striped size='small'>
        <Table.Body>{
        records.map(account =>
          <Table.Row key={account.key}>
            <Table.Cell width={10}>{account.key}</Table.Cell>
            <Table.Cell width={3}>{account.block}</Table.Cell>
            <Table.Cell width={3}>{account.memo}</Table.Cell>
            <Table.Cell width={3}>{account.address}</Table.Cell>
          </Table.Row>
        )}
        </Table.Body>
      </Table>
      {/* <div>{records}</div> */}
      </Form>
      <h1>Poe Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label={'Your file'}
            onChange={ (e) => handleFileChosen(e.target.files[0])}
          />
          <Input 
            type='text'
            id='memo'
            label='memo'
            onChange = { (e) => setMemo(e.target.value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton 
            accountPair={accountPair}
            label='create claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaimWithMemo',
              inputParams: [digest, memo],
              paramFields:[true, true]
            }}
          />
        </Form.Field>
          <div>{`Claim Info owner: ${owner} blockNumber ${blockNumber} Memo ${memo} Digest`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
