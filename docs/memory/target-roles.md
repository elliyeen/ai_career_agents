# Target Roles

Defines exactly which roles the system is allowed to pursue.
Agents must match a job to at least one entry here before qualifying it.

---

## Primary Targets

These are the roles you actively want. Score weight: full.

| Title Pattern | Min Seniority | Max Seniority | Notes |
|---------------|---------------|---------------|-------|
| [Title 1]     | [Senior]      | [Director]    | [Any notes] |
| [Title 2]     | [Staff]       | [Principal]   | [Any notes] |
| [Title 3]     | [Lead]        | [VP]          | [Any notes] |

---

## Acceptable Alternatives

Roles you would consider if the scope and compensation are right.
Score weight: 75%.

| Title Pattern | Condition to accept |
|---------------|---------------------|
| [Title]       | [e.g., "Only if remote and $X+ base"] |
| [Title]       | [...] |

---

## Excluded Titles

Do not qualify any job matching these patterns. Hard disqualify.

- [Title or keyword — e.g., "Junior", "Associate", "Intern"]
- [Title or keyword]
- [Title or keyword]

---

## Keywords That Qualify a Job Title

If a job title contains any of these words, it may qualify:

- [keyword]
- [keyword]
- [keyword]

---

## Keywords That Disqualify a Job Title

If a job title contains any of these words, it is disqualified immediately:

- [keyword — e.g., "Sales", "Marketing", "Accounting"]
- [keyword]

---

## Seniority Mapping

| IC Level   | Management Level | Years Experience |
|------------|-----------------|------------------|
| Senior     | Manager         | 5+               |
| Staff      | Senior Manager  | 8+               |
| Principal  | Director        | 10+              |
| Distinguished | VP            | 15+              |

Target minimum: [YOUR MINIMUM]
Hard floor: [ABSOLUTE MINIMUM — below this, auto-disqualify]
