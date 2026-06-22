// Screenshot-only mock of @tauri-apps/api/core.
// Returns realistic data so the UI renders populated for design screenshots.
// Never bundled into the real app — only used by vite.screenshot.config.ts.

const PROFILES = [
  {
    name: "work",
    auth_type: "oauth",
    description: "Primary work account",
    is_active: true,
    color: "",
    sessions: ["backend", "frontend"],
  },
  {
    name: "personal",
    auth_type: "api",
    description: "Personal projects",
    is_active: false,
    color: "",
    sessions: ["default"],
  },
  {
    name: "team-shared",
    auth_type: "api",
    description: "Shared team pool",
    is_active: false,
    color: "",
    sessions: ["default"],
  },
  {
    name: "research",
    auth_type: "vertex",
    description: "Vertex AI experiments",
    is_active: false,
    color: "",
    sessions: [],
  },
];

const ACTIVE = { profile: "work", session: "backend" };

const STATS = [
  {
    profile: "work",
    session: "backend",
    session_count: 42,
    rate_limit_hits: 1,
    key_rotations: 0,
    tokens_in: 1_284_000,
    tokens_out: 318_500,
    estimated_cost_usd: 7.42,
    first_used: "2026-05-01T09:00:00Z",
    last_used: "2026-06-22T14:12:00Z",
  },
  {
    profile: "work",
    session: "frontend",
    session_count: 28,
    rate_limit_hits: 0,
    key_rotations: 0,
    tokens_in: 642_000,
    tokens_out: 151_200,
    estimated_cost_usd: 3.91,
    first_used: "2026-05-03T11:00:00Z",
    last_used: "2026-06-21T18:40:00Z",
  },
  {
    profile: "personal",
    session: "default",
    session_count: 11,
    rate_limit_hits: 0,
    key_rotations: 2,
    tokens_in: 208_400,
    tokens_out: 47_900,
    estimated_cost_usd: 1.12,
    first_used: "2026-05-10T08:00:00Z",
    last_used: "2026-06-20T10:05:00Z",
  },
  {
    profile: "team-shared",
    session: "default",
    session_count: 63,
    rate_limit_hits: 4,
    key_rotations: 5,
    tokens_in: 2_910_000,
    tokens_out: 690_000,
    estimated_cost_usd: 16.88,
    first_used: "2026-04-18T08:00:00Z",
    last_used: "2026-06-22T09:30:00Z",
  },
];

const SWITCH_LOG = [
  {
    timestamp: "2026-06-22T09:00:00Z",
    from_profile: "team-shared",
    from_session: "default",
    to_profile: "work",
    to_session: "backend",
    reason: "Schedule",
    detail: "active_hours start (09:00)",
  },
  {
    timestamp: "2026-06-21T18:00:00Z",
    from_profile: "work",
    from_session: "backend",
    to_profile: "team-shared",
    to_session: "default",
    reason: "Schedule",
    detail: "active_hours end (18:00)",
  },
];

const SCHEDULER = [
  {
    profile: "team-shared",
    detected_at: "2026-06-22T08:40:00Z",
    refill_at: "2026-06-22T14:40:00Z",
    time_until_refill: "23m",
    auto_switch_back: true,
    switched_back: false,
  },
];

const RESPONSES: Record<string, unknown> = {
  list_profiles: PROFILES,
  get_active: ACTIVE,
  daemon_status: { running: true, active_timers: 1 },
  get_switch_log: SWITCH_LOG,
  get_scheduler_state: SCHEDULER,
  get_stats: STATS,
};

export async function invoke<T>(cmd: string): Promise<T> {
  if (cmd in RESPONSES) return RESPONSES[cmd] as T;
  return undefined as T;
}
