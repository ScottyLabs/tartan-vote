# JSON Information

- [vote.data](db-json.md#votedata)
- [motion.data](db-json.md#motiondata)
- [organization.data](db-json.md#organizationdata)
- [log.data](db-json.md#logdata)

## `vote.data`

The `vote.data` field stores the submitted voting payload for a single vote record.

### Type Definition

```typescript
type VoteData = {
    proxy: boolean;
    proxy_for_user_id: number | null;
    vote_response: string[];
};
```

### Field Descriptions

- `proxy`: Whether this vote was cast as a proxy vote instance.
- `proxy_for_user_id`: If `proxy` is true, this stores the proxied user's id; otherwise `null`.
- `vote_response`: Stores the participant's submitted response as an array.
  - For a standard single-choice motion, this array typically contains a single value.
  - For a ranked-choice ballot, this array stores the ranked selections in order of preference.
  - A lower array index indicates a higher preference (e.g., index 0 = first choice, index 1 = second choice).

### Example Usage

```json
{
    "proxy": false,
    "proxy_for_user_id": null,
    "vote_response": ["choice1"]
}
```

## `motion.data`

The `motion.data` field stores motion-specific configuration and metadata.

### Type Definition

```typescript
type MotionData = {
    description: string;
    session_code: string;
    visibility: {
        participants: string;
    };
    proxy: bool;
    vote_options: string[];
};
```

### Field Descriptions

- `description`: A textual description of the motion.
- `session_code`: A code used for joining or identifying the session.
- `visibility.participants`: Defines what participants can see during the motion. Example values include `"hidden_until_release"` and `"live"`.
- `proxy`: Indicates whether proxy voting is enabled for the motion.
- `vote_options`: Lists the selectable voting options for the motion.

### Example Usage

```json
{
    "description": "Motion description goes here",
    "session_code": "code",
    "visibility": {
        "participants": "hidden_until_release"
    },
    "proxy": true,
    "vote_options": ["option1", "option2"]
}
```

### Notes

- For now, all users are treated as having the same role.

## `organization.data`

The `organization.data` field stores organization-level metadata.

### Type Definition

```typescript
type OrganizationData = {
    description: string;
};
```

### Field Descriptions

- description: A textual description of the organization.

### Example Usage

```json
{
    "description": "Organization Description"
}
```

## `log.data`

The `log.data` field stores audit log information for system actions. Logging has not been implemented yet, so this section is not in use currently.

### Type Definition

```typescript
type LogData = {
    action: string;
    target: {
        table: string;
        id: number;
    };
    actor:
        | {
              user_id: number;
              role: string;
          }
        | "system";
    event_id: number;
    changes: {
        before: {};
        after: {};
    };
};
```

### Field Descriptions

- `action`: A description of the action being logged.
- `target`: Identifies the record affected by the action.
- `table`: The name of the affected table.
- `id`: The primary key of the affected record.
- `actor`: Identifies the user responsible for the action.
- `user_id`: The ID of the acting user.
- `role`: The role of the acting user.
- `event_id`: The related event identifier, if applicable.
- `changes`: Stores the state transition caused by the action.
- `before`: The state before the change.
- `after`: The state after the change.

### Example Usage

```json
{
    "action": "Action Description",
    "target": {
        "table": "tablename",
        "id": 0
    },
    "actor": {
        "user_id": 0,
        "role": "Role String"
    },
    "event_id": 0,
    "changes": {
        "before": {},
        "after": {}
    }
}
```
