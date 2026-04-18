export class Vote {
    id: number;
    cast_time: string;
    data: VoteData;

    constructor(data: {
        id: number;
        cast_time: string;
        data: VoteData;
    }) {
        this.id = data.id;
        this.cast_time = data.cast_time;
        this.data = data.data;
    }
}
