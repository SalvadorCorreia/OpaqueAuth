#OpaqueAuth
OpaqueAuth is an open-source project that aims to bring state-of-the-art, zero-trust password verification to all web appliclations.
This will be achieved through the development of a client-side WebAssembly environment without backend telemetry.

## Contextualization
In recent years, the landscape of password strength evaluation has irrevocably shifted.
This section documents this structural paradigm shift and introduces the current state-of-the-art for password strength verification, justifying the technologies selected for our implementation. 

### Password Strength Metrics
The leading cause for the shift in methodology was, of course, the complete overhaul of the metric system.
For years, we all interacted with arbitrary complexity requirements—forcing us to combine uppercase letters, lowercase letters, numbers and symbols.
These constraints were introduced to prevent brute-force attacks, which were assumed to simply follow all possible character combinations.
Previous evaluation standards were built on the same logic, essentially equating entropy to strength.

However, empirical data has conclusively demonstrated that these constraints resulted in predictable behaviours, such as capitalization of the initial letter or appending a single digit or symbol at the end of a password.
Furthermore, modern password cracking technology takes advantege of the false sense of security previous systems provided, exploiting these predictable behaviours.

Given the ineffective nature of the previous metric system, a new one was introduced: "guessability".
Mathematically speaking, this concept is formalized as the number of attempts an optimal adversary would need to discover the password, operating under the assumption that the adversary sorts all possible passwords in descending order of their probability.
This system provides more realistic measurements, mirroring the real threat model.
