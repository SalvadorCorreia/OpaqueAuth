# OpaqueAuth
OpaqueAuth is an open-source project that aims to bring state-of-the-art, zero-trust password verification to all web applications. This will be achieved through the development of a client-side WebAssembly environment without backend telemetry.

## Contextualization
In recent years, the landscape of password strength evaluation has irrevocably shifted. This section documents this structural paradigm shift and introduces the current state-of-the-art for password strength verification, justifying the technologies selected for our implementation. 

### Password Strength Metrics
The leading cause for the shift in methodology was, of course, the complete overhaul of the metric system. For years, we all interacted with arbitrary complexity requirements—forcing us to combine uppercase letters, lowercase letters, numbers, and symbols [1]. These constraints were introduced to prevent brute-force attacks, which were assumed to simply follow all possible character combinations. Previous evaluation standards were built on the same logic, essentially equating entropy to strength [2].

However, empirical data has conclusively demonstrated that these constraints resulted in predictable behaviours, such as capitalization of the initial letter or appending a single digit or symbol at the end of a password [1]. Furthermore, modern password cracking technology takes advantage of the false sense of security previous systems provided, exploiting these predictable behaviours [1].

Given the ineffective nature of the previous metric system, a new one was introduced: "guessability". Mathematically speaking, this concept is formalized as the number of attempts an optimal adversary would need to discover the password, operating under the assumption that the adversary sorts all possible passwords in descending order of their probability [3]. This system provides more realistic measurements, mirroring the real threat model.

### Standards
This drastic change can also be seen in standards such as NIST Special Publication 800-63B Revision 4 (Digital Identity Guidelines: Authentication and Authenticator Management). The mechanisms of password validation have been completely overhauled [4]: 

| Policy Element | Legacy Paradigm (Pre-2024) | NIST SP 800-63B Rev 4 Paradigm | Rationale |
|---|---|---|---|
| Length Minimum | 8 characters | 15 characters (8 if using multi-factor) | Entropy exponentially scales with length, mitigating offline brute-force attacks. |
| Length Maximum | Often restricted (e.g., 16-20) | At least 64 characters | Facilitates long, highly entropic passphrases that resist probabilistic modeling. |
| Composition | Mandatory mix of types (A, a, 1, !) | No composition rules allowed | Forced complexity creates predictable, algorithmic human substitution patterns. |
| Expiration | 30 to 90 days | Only upon known compromise | Frequent rotation forces minor iterations (e.g., Pass1 to Pass2), degrading security. |
| Screening | None | Mandatory breached blocklist | Protects against credential stuffing by eliminating known vulnerability points. |

This paradigm shift is further reinforced by the OWASP Top 10:2025 guidelines, which maintain "Authentication Failures" as a critical vulnerability vector.
Echoing the NIST specifications, OWASP mandates the screening of passwords against known breached credentials and common dictionaries[5].

[1] Melicher, W., Ur, B., Segreti, S. M., Bauer, L., Christin, N., & Cranor, L. F. (2017). Better passwords through science (and neural networks). *USENIX ;login: Winter 2017*, 42(4).
[2] Wheeler, D. (2012). zxcvbn: realistic password strength estimation. *Dropbox.Tech*. 
[3] de Castro, L., Lang, H., Liu, S., & Mata, C. (2017). Modeling Password Guessing with Neural Networks. *Massachusetts Institute of Technology*.
[4] Temoshok, D., Fenton, J. L., Choong, Y.-Y., Lefkovitz, N., Regenscheid, A., Galluzzo, R., & Richer, J. P. (2025). Digital Identity Guidelines: Authentication and Authenticator Management. *NIST Special Publication 800-63B-4*.
[5] OWASP Foundation. (n.d.). Authentication Cheat Sheet. *OWASP Cheat Sheet Series*.
