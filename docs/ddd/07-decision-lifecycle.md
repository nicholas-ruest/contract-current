# Contract lifecycle

## Model
Contract ChangeContract transitions Draft, Issued, Active, Completed or Refused; evidence is immutable.

## Invariants
This context owns validation and transitions. Cross-context work uses typed values and explicit ports. Failed invariants return domain errors and cause no hidden durable effect.

## Events and verification
Events carry API revision or contract id plus correlation id. Focused tests verify invariants; the acceptance map verifies the context end to end.
