# Protocol overview

Primar settles agent-to-agent service calls on Stellar using three Soroban contracts:

1. **Registry** — providers register a service id, price per call, and lifecycle status.
2. **Budget** — operators set session/task caps so agents cannot overspend.
3. **Settlement** — payers record settlements with auth; protocol fee is computed in basis points.

All three contracts support `initialize`, typed `contracterror` codes, and admin pause/unpause in v2.
