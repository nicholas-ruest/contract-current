# Research

## Sources
- RuVector c44d7337c0c5ad2ecd2294178475387facb96516: real VectorDB insert/search recalls successful migration plan compositions.
- RVF Runtime 0.3.2: append/reopen/read is the sole contract and audit persistence boundary.
- AWS Cedar 2c2b84772a63e8a5d57d98037b9389c5fd3812a3: cedar-policy 4.14.0 authorizes migration windows before persistence.
- Microservice API Evolution in Practice, DOI:10.1016/j.jss.2024.112110: manual consumer-impact analysis is error-prone and communication is incomplete.
- ReCIRC, arXiv:2609.38112: input-adaptive conformal risk budgets preserve a finite-sample marginal guarantee.

## Hypothesis
Combining ReCIRC calibration, policy execution, analogue recall and durable receipts will form safer migration plans than structural diff alone. Removing calibration lowers safe coverage; removing AWS Cedar admits a forbidden frozen-window migration; removing recall reduces utility; removing RVF removes restart recovery.

Consumer observations are deterministic synthetic fixtures. The measured capability is governed migration plan formation, not live-model quality.
