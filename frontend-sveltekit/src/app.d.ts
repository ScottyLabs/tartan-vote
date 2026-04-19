// See https://kit.svelte.dev/docs/types#app
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface Platform {}
  }

  // Domain types (mirrors the original app.d.ts)
  interface EventData {
    description?: string;
    session_code: string;
    vote_type: 'motion' | 'election';
    threshold: number;
    visibility: { participants: 'hidden_until_release' | 'live' };
    proxy: boolean;
    vote_options: string[];
    eligible_voter_user_ids?: number[];
    proxy_assignments?: Array<{
      proxy_holder_user_id: number;
      proxied_senator_user_id: number;
    }>;
    ballot_style?: 'standard' | 'secret';
    meeting_display?: 'named_by_category' | 'totals_only';
    export_scope?: 'totals_only' | 'full_ballots';
    anonymous?: boolean;
  }

  interface VoteData {
    vote_type: string;
    vote_response: string[];
    proxy?: boolean;
    proxy_for_user_id?: number;
  }

  interface Time {
    days: number;
    hours: number;
    mins: number;
    secs: number;
  }
}

export {};
