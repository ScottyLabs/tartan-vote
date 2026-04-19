export class Vote {
  id: number;
  cast_time: string;
  data: VoteData;

  constructor(init: Partial<Vote> = {}) {
    this.id = init.id ?? 0;
    this.cast_time = init.cast_time ?? '';
    this.data = (init.data ?? { vote_type: 'motion', vote_response: [] }) as VoteData;
  }
}
