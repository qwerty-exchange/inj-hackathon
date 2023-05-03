export enum PropositionType {
  Ask = 'ask',
  Bid = 'bid'
}

export enum PropositionState {
  Active = 'active',
  Accepted = 'accepted',
  Closed = 'closed',
  Rejected = 'rejected'
}

type Coin = {
  denom: string
  amount: string
}

export type Proposition = {
  id?: string
  owner?: string
  proposition_type: PropositionType
  state?: PropositionState
  deposit: Coin
  assets: Coin
  premium: Coin
  period: number
  expiry: number
  contractor?: string
}
