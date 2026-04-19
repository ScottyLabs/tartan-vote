export class User {
  id: number;
  name: string;
  andrew_id: string;
  oidc_client: string;
  created_at: string;

  constructor(init: Partial<User> = {}) {
    this.id = init.id ?? 0;
    this.name = init.name ?? '';
    this.andrew_id = init.andrew_id ?? '';
    this.oidc_client = init.oidc_client ?? '';
    this.created_at = init.created_at ?? '';
  }
}
