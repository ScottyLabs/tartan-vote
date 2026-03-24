export class User {
    id: number;
    name: string;
    andrew_id: string;
    oidc_client: string;
    created_at: string;

    constructor(data: {
        id: number;
        name: string;
        andrew_id: string;
        oidc_client: string;
        created_at: string;
    }) {
        this.id = data.id;
        this.name = data.name;
        this.andrew_id = data.andrew_id;
        this.oidc_client = data.oidc_client;
        this.created_at = data.created_at;
    }
}
