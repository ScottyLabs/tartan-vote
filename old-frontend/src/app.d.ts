interface Event {
  id: number;
  event_type: string;
  name: string;
  status: string;
  start_time: string;
  end_time: string | null;
  data: EventData;
  created_by_user_id: number;
  organization_id: number;
}

interface EventData {
  description: string;
  session_code: string;
  vote_type: "motion" | "election";
  threshold: number; // float, not string
  visibility: {
    participants: "hidden_until_release" | "live";
  };
  proxy: boolean;
  vote_options: string[];
  eligible_voter_user_ids?: number[];
  proxy_assignments?: {
    proxy_holder_user_id: number;
    proxied_senator_user_id: number;
  }[];
}

interface User {
  id: number;
  name: string;
  andrew_id: string;
  oidc_subject: string;
  created_at: string;
}

interface Vote {
  id: i32;
  cast_time: string;
  data: Json;
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
