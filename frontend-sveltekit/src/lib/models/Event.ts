export class Event {
  id: number;
  event_type: string;
  name: string;
  status: string;
  start_time: string;
  end_time: string | null;
  data: EventData;
  created_by_user_id: number;
  organization_id: number;

  constructor(init: Partial<Event> = {}) {
    this.id = init.id ?? 0;
    this.event_type = init.event_type ?? 'Motion';
    this.name = init.name ?? '';
    this.status = init.status ?? 'pending';
    this.start_time = init.start_time ?? '';
    this.end_time = init.end_time ?? null;
    this.data = (init.data ?? {
      session_code: '',
      vote_type: 'motion',
      threshold: 0.5,
      visibility: { participants: 'live' },
      proxy: false,
      vote_options: ['Pass', 'Reject', 'Abstain']
    }) as EventData;
    this.created_by_user_id = init.created_by_user_id ?? 0;
    this.organization_id = init.organization_id ?? 0;
  }

  isMotion(): boolean {
    return this.event_type.toLowerCase() === 'motion';
  }
  isElection(): boolean {
    return this.event_type.toLowerCase() === 'election';
  }
  isExpired(): boolean {
    if (!this.end_time) return false;
    return new Date(this.end_time).getTime() <= Date.now();
  }
  timeUntilEnd(): Time | null {
    if (!this.end_time) return null;
    const ms = new Date(this.end_time).getTime() - Date.now();
    if (ms <= 0) return { days: 0, hours: 0, mins: 0, secs: 0 };
    const secs = Math.floor(ms / 1000) % 60;
    const mins = Math.floor(ms / (1000 * 60)) % 60;
    const hours = Math.floor(ms / (1000 * 60 * 60)) % 24;
    const days = Math.floor(ms / (1000 * 60 * 60 * 24));
    return { days, hours, mins, secs };
  }
}
