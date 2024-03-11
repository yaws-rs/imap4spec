# IMAP

This was quickly written for a client MVP and probably isn't safe yet to use in prod - no fuzzing etc done.

IANA Capabilities Registry

- https://www.iana.org/assignments/imap-capabilities/imap-capabilities.xhtml

## Core

| RFC    | Description | Done? |
| :---   | :---        | :---  |
| [3501](https://www.rfc-editor.org/rfc/rfc3501)        | 2003 IMAP 4rev1  | :shrug: |
| [9051](https://datatracker.ietf.org/doc/html/rfc9051) | 2021 IMAP 4rev2  | :shrug: |
|

## Extensions

| RFC    | Description | Done? |
| :---   | :---        | :---  |
| [2087](https://www.rfc-editor.org/rfc/rfc2087)        | 1997 IMAP4 QUOTA | :shrug  |
| [2971](https://www.rfc-editor.org/rfc/rfc2971)        | 2000 ID | :shrug: |
| [4314](https://www.rfc-editor.org/rfc/rfc4314)        | 2005 Access Control List (ACL) | :shrug |
| [4315](https://www.rfc-editor.org/rfc/rfc4315)        | 2005 UIDPLUS | :shrug: |
| [4551](https://www.rfc-editor.org/rfc/rfc4551)        | 2006 Conditional STORE Operation or Quick Flag Changes Resynchronization | :shrug: |
| [5161](https://www.rfc-editor.org/rfc/rfc5161)        | 2008 ENABLE | :shrug: |
| [5256](https://www.rfc-editor.org/rfc/rfc5256)        | 2008 SORT and THREAD | :shrug: |
| [5456](https://www.rfc-editor.org/rfc/rfc5464)        | 2009 METADATA | :shrug: |
| [5465](https://www.rfc-editor.org/rfc/rfc5465)        | 2009 NOTIFY   | :shrug: |
| [7162](https://www.rfc-editor.org/rfc/rfc7162)        | 2014 CONDSTORE + QRESYNC | :shrug: |

## Fuzzing

Must. Need. Got an interngalactic spaceship to sponsor to shoot bugs ?