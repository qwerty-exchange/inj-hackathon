import {
  NotifiClient,
  createGraphQLClient,
  createNotifiService
} from '@notifi-network/notifi-node'
import { bech32 } from 'bech32'
import { randomUUID } from 'crypto'
import WebSocket from 'ws'

export const getEthereumAddress = (injectiveAddress: string): string => {
  if (injectiveAddress.startsWith('0x')) {
    return injectiveAddress
  }

  return `0x${Buffer.from(
    bech32.fromWords(bech32.decode(injectiveAddress).words)
  ).toString('hex')}`
}

export const sendMessage = async (address: string, message: string) => {
  const graphqlClient = createGraphQLClient('Development')
  const notifiService = createNotifiService(graphqlClient)

  const client = new NotifiClient(notifiService)
  const result = await client.logIn({
    sid,
    secret
  })

  await client.sendDirectPush(result.token, {
    key: randomUUID(),
    type: 'qwertyexchange__cryptopawn',
    walletPublicKey: getEthereumAddress(address).toLowerCase(),
    walletBlockchain: 'ETHEREUM' as any,
    message
  })
}

const ws = new WebSocket('wss://k8s.testnet.tm.injective.network/websocket')
const contractAddress = process.env.contractAddress!
const sid = process.env.notifi_sid!
const secret = process.env.notifi_secret!


ws.onopen = function () {
  // Subscribe to new block events
  const subscribeMsg = JSON.stringify({
    jsonrpc: '2.0',
    id: '1',
    method: 'subscribe',
    params: {
      query: `tm.event='Tx' and wasm._contract_address='${contractAddress}}'`
    }
  })
  ws.send(subscribeMsg)
}

ws.onmessage = async function  (event) {
  const msg = JSON.parse(event.data.toString())
  if (msg.result && msg.result.data && msg.result.data.value) {
    const obj = Object.fromEntries(
      msg.result.data.value.TxResult.result.events
        .find((x: any) => x.type === 'wasm')
        .attributes.map((x: any) => [
          Buffer.from(x.key, 'base64').toString('utf-8'),
          Buffer.from(x.value, 'base64').toString('utf-8')
        ])
    )
    console.log(obj);
    if (obj.method == 'accept_proposition') {
      await sendMessage(obj.owner, 'Your proposition has been accepted')
    }

    if (obj.method == 'close_proposition') {
      await sendMessage(obj.owner, 'Your proposition has been closed')
      await sendMessage(obj.contractor, 'Your proposition has been closed')
    }
  }
}

ws.onclose = function (event) {
  console.log('WebSocket connection closed')
  process.kill(1)
}