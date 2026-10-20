# Security

Rodro Turbo Search is a functional MVP. There is no stable release line, formal
security audit, or guaranteed security response time. Only the current development
state is maintained on a best-effort basis.

## Reporting a vulnerability

Do not include exploitable details, credentials, private filenames, or personal
logs in a public issue. If the repository's **Security → Report a vulnerability**
option is available, use GitHub's private vulnerability reporting. This channel
must first be enabled by the maintainer; it is not assumed to be active.

If that option is absent, open an issue asking the maintainer to enable a private
reporting channel, without describing the vulnerability or including sensitive
data. No dedicated security email address is configured.

A private report should include the affected commit/build, environment,
reproduction steps, impact, and a suggested fix if available. Use harmless sample
files rather than real private data.

## Local data

Search queries and the selected mode stay in process memory. The application
searches application metadata and, optionally, filenames under the home directory;
it does not read file contents for search. A successful calculator activation
writes its result to the system clipboard. Opening results invokes GIO and the
user's desktop handlers, which are separate programs with their own behavior.
The installer writes desktop integration files and uninstall state in the user's
XDG directories. None of those files should be committed.

## En español

No publiques detalles de vulnerabilidades ni datos privados en issues. Usá
**Security → Report a vulnerability** si está habilitado. Si no aparece, pedí
habilitar un canal privado mediante un issue sin información sensible. Este MVP
se mantiene según disponibilidad y no tiene un plazo garantizado de respuesta.
