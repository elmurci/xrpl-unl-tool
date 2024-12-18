# XRP Ledger Validators List Tool

Tool to validate and sign Validators Lists.

## Description

This tool allows you to `load` and `sign` VL's and `encode` and `decode` manifests.

### Versions supported

The tool supports both version 1 and 2 of the VL format.
For more information about v2, follow this [link](https://github.com/XRPLF/XRPL-Standards/tree/master/XLS-0045-prepublish-validator-lists).

### Load

The command allows for either a url or a file path:

`./xrpl-vl-tool load {url_or_path}`

And loads the given VL performing some validations.

#### Example

*Example request:*

`./xrpl-vl-tool load https://vl.xrplf.org` or `./xrpl-vl-tool load /dev/vl.json`

*Example response:*

```
There are 1 UNL's in this Validators List | Version 2 | Manifest Signature: ✓


1) There are 35 validators in this VL. Sequence is: 81 | Blob Signature: x | Effective from: 2024-09-05 23:56:00 | Expires: 2025-12-04 22:53:42 

Validator: ED13AAFCB6A87BCB5D093C2EF37F04431C291126D674293305152D9776C6ABA4D6 (nHBWa56Vr7csoFcCnEPzCCKVvnDQw3L28mATgHYQMGtbEfUjuYyB) | Master: ✓, Signing: ✓ | xrp.vet
Validator: ED4246AA3AE9D29863944800CCA91829E4447498A20CD9C3973A6B59346C75AB95 (nHBidG3pZK11zQD6kpNDoAhDxH6WLGui6ZxSbUx7LSqLHsgzMPec) | Master: ✓, Signing: ✓ |  
Validator: ED5784A43AA84B5BDAFD0AFEF64ADA5583A3129182C6A7464950FD6BF2D9FAE5B0 (nHUryiyDqEtyWVtFG24AAhaYjMf9FRLietbGzviF3piJsMm9qyDR) | Master: ✓, Signing: ✓ |  
Validator: ED583ECD06C3B7369980E65C78C440A529300F557ED81256283F7DD5AA3513A334 (nHUpJSKQTZdB1TDkbCREMuf8vEqFkk84BcvZDhsQsDufFDQVajam) | Master: ✓, Signing: ✓ |  
Validator: ED65142881189CA8FE8D246A8EACE7637A8CA7CE78656638C6D87FAD369F8A5C81 (nHUfxETNHsA9reyYCVYwNztEbifMg6U9YUdcgVvzMwGNpphKSSf6) | Master: ✓, Signing: ✓ | xrpkuwait.com
Validator: ED7098772471769E82A5466329967DC8BF51C941190164E88D7CC9C393AD407C52 (nHUDpRzvY8fSRfQkmJMqjmVSaFmMEVxBNn2tNQy5VAhFJ6is6GFk) | Master: ✓, Signing: ✓ | ekiserrepe.es
Validator: ED8252C2F91523126EEF9A21964C7E487A10D6D63D459139700DBC70D9F7BAD542 (nHULqGBkJtWeNFjhTzYeAsHA3qKKS7HoBh8CV3BAGTGMZuepEhWC) | Master: ✓, Signing: ✓ |  
Validator: EDA4074FD039407BD2464F14C378440D5B02CA8FBA661B286D1C82A3D59E8E6EC0 (nHUbgDd63HiuP68VRWazKwZRzS61N37K3NbfQaZLhSQ24LGGmjtn) | Master: ✓, Signing: ✓ |  
Validator: EDFE65FB385B6BB16951153D2A0F32BD6D8CC4532C87BB3E1900913A7BE34F5EF7 (nHDH7bQJpVfDhVSqdui3Z8GPvKEBQpo6AKHcnXe21zoD4nABA6xj) | Master: ✓, Signing: ✓ |  
Validator: EDC1897CE83B6DCF58858574EC9FE027D4B1538A0F20823800A5529E121E87A93B (nHUFCyRCrUjvtZmKiLeF8ReopzKuUoKeDeXo3wEUBVSaawzcSBpW) | Master: ✓, Signing: ✓ |  
Validator: EDC2A138B3771C208965596D4D372331C17A5476BD2CE2BC7A6D3CD273DF330D99 (nHUq9tJvSyoXQKhRytuWeydpPjvTz3M9GfUpEqfsg9xsewM7KkkK) | Master: ✓, Signing: ✓ |  
Validator: ED38B0288EA240B4CDEC18A1A6289EB49007E4EBC0DE944803EB7EF141C5664073 (nHB8QMKGt9VB4Vg71VszjBVQnDW3v3QudM4DwFaJfy96bj4Pv9fA) | Master: ✓, Signing: ✓ | bithomp.com
Validator: ED135050AE848C37B894EFC67BBEC54A5B4CBAA2281C9DB2D7754A3DF6195DA65E (nHBVACxZaNbUjZZkBfj7gRxF3xgG2vbcP4m48KzVwntdTogi5Tfs) | Master: ✓, Signing: ✓ | onxrp.com
Validator: ED2C5C95F6B67357282B7F1675AFBBAACFB61DF06DEEDF986166E7ADD3D7D33462 (nHBgyVGAEhgU6GoEqoriKmkNBjzhy6WJhX9Z7cZ71yJbv28dzvVN) | Master: ✓, Signing: ✓ | v2.xrpl-commons.org
Validator: ED571031CEB567106CD4E128D46E4DD4087DA12DA9FAB78EEFF7A93971DCC59900 (nHUr8EhgKeTc9ESNt4nMYzWC2Pu7GgRHMRTsNEyGBTCfnHPxmXcm) | Master: ✓, Signing: ✓ | anodos.finance
Validator: ED6A54975A94EB9715E4F4E3FCD1661FCD40C065E6C22E461FEE87267DD73A2D6A (nHUwGQrfZfieeLFeGRdGnAmGpHBCZq9wvm5c59wTc2JhJMjoXmd8) | Master: ✓, Signing: ✓ | xrpgoat.com
Validator: ED95C5172B2AD7D39434EEBC436B65B3BB7E58D5C1CEFC820B6972ACAD776E286A (nHUVPzAmAmQ2QSc4oE1iLfsGi17qN2ado8PhxvgEkou76FLxAz7C) | Master: ✓, Signing: ✓ | ripple.ittc.ku.edu
Validator: ED9DA743B769045A91AC41CA5C56FBD090168CB771E9558DD9D1C4FE8B3F4C842E (nHUY14bKLLm72ukzo2t6AVnQiu4bCd1jkimwWyJk3txvLeGhvro5) | Master: ✓, Signing: ✓ | validator.gatehub.net
Validator: EDA1EFC81058EECB48DEB4FEB7FAFACEAEA42C3E00C0BFB31F85EC116F31A13DAD (nHU2k8Po4dgygiQUG8wAADMk9RqkrActeKwsaC9MdtJ9KBvcpVji) | Master: ✓, Signing: ✓ | verum.eminence.im
Validator: EDAF4CBCF4A9BEE306646549301E22770D5E62D8C03DD9FF42B65A83B2BE1C70F3 (nHUge3GFusbqmfYAJjxfKgm2j4JXGxrRsfYMcEViHrFSzQDdk5Hq) | Master: ✓, Signing: ✓ | katczynski.net
Validator: EDF10074F5FBBB975A8EA8E9C42306854E6A49C71B7D33B0293AB1830FECF2C400 (nHDB2PAPYqF86j9j3c6w1F1ZqwvQfiWcFShZ9Pokg9q4ohNDSkAz) | Master: ✓, Signing: ✓ | xrpscan.com
Validator: ED55BB5A2C8E040367DBA0BA563E924463C360C9C565EFE41701596BA2B828DE16 (nHUrUNXCy4DgPPNABX9C6mUctpoq7CwgLKAUxjw6zYtTfiqsj1ew) | Master: ✓, Signing: ✓ | xrp-validator.interledger.org
Validator: ED580C4282950CB3F7E0185F37F2CFB216882C5EDDD3BB1EE49C304A1AA3C5DB92 (nHUpDPFoCNysckDSHiUBEdDXRu2iYLUgYjTzrj3bde5iDRkNtY8f) | Master: ✓, Signing: ✓ | validator.poli.usp.br
Validator: ED8815A1E647DF83DE643289804F4610464D1ABCF6B38F40404EFA84B5D1A69D81 (nHUP4RcLQdPHh3kMtFm9NFGnjEYLGXiQAyyB7qFsjATHMw2YVxHi) | Master: ✓, Signing: ✓ | xpmarket.com
Validator: EDA8B1D8A071E85A6E36DCAC7999AB814E4CDC664668385173767C5B89554243C0 (nHUdjQgg33FRu88GQDtzLWRw95xKnBurUZcqPpe3qC9XVeBNrHeJ) | Master: ✓, Signing: ✓ | validator.xrpl.robertswarthout.com
Validator: EDCAD6E02AAFF5467465CBB9E62E021BF4B8E23F7484A6F0F67387549733865CCA (nHUtmbn4ALrdU6U8pmd8AMt4qKTdZTbYJ3u1LHyAzXga3Zuopv5Y) | Master: ✓, Signing: ✓ | bifrostwallet.com
Validator: ED63CF929BE85B266A66584B3FE2EB97FC248203F0271DC9C833563E60418E7818 (nHUfPizyJyhAJZzeq3duRVrZmsTZfcLn7yLF5s2adzHdcHMb9HmQ) | Master: ✓, Signing: ✓ | xrp.unic.ac.cy
Validator: EDCF08053DFF0F00AC6E78B61F7B7FD187AF74052DEB5074207506D3A2CDCD9E5C (nHUvcCcmoH1FJMMC6NtF9KKA4LpCWhjsxk2reCQidsp5AHQ7QY9H) | Master: ✓, Signing: ✓ | jon-nilsen.no
Validator: ED580AD4FA5DA989FA999535ECC20197A5B53A1A49A971F6652ED8D5D466CA605D (nHUpDEZX5Zy9auiu4yhDmhirNu6PyB1LvzQEL9Mxmqjr818w663q) | Master: ✓, Signing: ✓ | xspectar.com
Validator: ED6753539020782A777B8F4BF6931A7DB13F9D259486E337C639B99E0C57CD5FF2 (nHU3AenyRuJ4Yei4YHkh6frZg8y2RwXznkMAomUE1ptV5Spvqsih) | Master: ✓, Signing: ✓ | xrpl.aesthetes.art
Validator: EDFF91FA911FE9BF2CCCC2A1F750900C4A6056139FE0DD6872D2A577CA51B27200 (nHDHzXZKtmMHCkTVgdWY4dqdigDrESiseUF8JkzE93DUtfbt6s3W) | Master: ✓, Signing: ✓ | validator.aspired.nz
Validator: ED9AE4F5887BA029EB7C0884486D23CF281975F773F44BD213054219882C411CC7 (nHUXeusfwk61c4xJPneb9Lgy7Ga6DVaVLEyB29ftUdt9k2KxD6Hw) | Master: ✓, Signing: ✓ | validator.xrpl-labs.com
Validator: ED8651B672BCE2727BD93A62431592447D6637E5D0E768595ECC19E5E4AEACAF3B (nHU4bLE3EmSqNwfL4AP1UZeTNPrSPPP6FXLKXo2uqfHuvBQxDVKd) | Master: ✓, Signing: ✓ | ripple.com
Validator: ED75940EC09130F9C553D8AF0FE354A112CC27251472AF1A90917597489192135F (nHUED59jjpQ5QbNhesXMhqii9gA8UfbBmv3i5StgyxG98qjsT4yn) | Master: ✓, Signing: ✓ | arrington-xrp-capital.blockdaemon.com
Validator: EDA54C85F91219FD259134B6B126AD64AE7204B81DD4052510657E1A5697246AD2 (nHUcNC5ni7XjVYfCMe38Rm3KQaq27jw7wJpcUYdo4miWwpNePRTw) | Master: ✓, Signing: ✓ | cabbit.tech
```

### Sign

Signs a new (or appends to an existing VL) UNL retrieving the secret from AWS.

`./xrpl-vl-tool sign {version} {publisher_manifest} {manifests_file} {sequence} {expiration_in_days} {secret_provider(aws or vault)} {secret_name} {effective_date(for v2)} {effective_time(for v2)} {v2_vl_file(optional, for v2)}`

#### AWS Secrets

In order to retrieve the secret from AWS, the following environment variables need to be available:

- `AWS_REGION`
- `AWS_ACCESS_KEY_ID`
- `AWS_SESSION_TOKEN`
- `AWS_SECRET_ACCESS`

`secret_name` example: `test/vl/tool`.

#### Vault Secrets

For Vault secrets, the following environment variables are required:

- `VAULT_TOKEN`
- `VAULT_ENDPOINT`

The format for secret name is `{mount}:{path}`.

`secret_name` example: `vl-tool/dev:keypair`.

#### Example

*Example request:*

`./xrpl-vl-tool sign 2 JAAAAAFxIe0md6v/0bM6xvvDBitx8eg5fBUF4cQsZNEa0bKP9z9HNHMh7V0AnEi5D4odY9X2sx+cY8B3OHNjJvMhARRPtTHmWnAhdkDFcg53dAQS1WDMQDLIs2wwwHpScrUnjp1iZwwTXVXXsaRxLztycioto3JgImGdukXubbrjeqCNU02f7Y/+6w0BcBJA3M0EOU+39hmB8vwfgernXZIDQ1+o0dnuXjX73oDLgsacwXzLBVOdBpSAsJwYD+nW8YaSacOHEsWaPlof05EsAg== test/data/manifest_1.txt 81 365 vault vl-tool/dev:keypair 2015-09-05 23:56 test/data/generated_vl_v2_2.json`

*Example response:*

`VL file generated ✓` (a timestamped json file saved to the current folder)

## Validator List fields

### Version 1

- `blob`: Base64-encoded JSON string containing `sequence`, `validators` and `expiration` fields.
    - `sequence`: Validator list sequence (incremental)
    - `expiration`: Ripple timestamp (seconds since January 1st, 2000 (00:00 UTC)) for when
        the list expires.
    - `validators` contains an array of objects with a hex `validation_public_key` and a base64-encoded `manifest`
- `manifest`: Base64-encoded serialization of a manifest containing the publisher's master and signing public keys.
- `signature`: Hex-encoded signature of the blob using the publisher's signing key.
- `version`: The version of the validator list protocol this object uses. The current version is 1. A higher version number indicates backwards-incompatible changes with a previous version of the validator list protocol.
- `public_key`: The public key used to verify this validator list data, in hexadecimal. This is a 32-byte Ed25519 public key prefixed with the byte 0xED. The value is equal to the `master_public_key` in the publisher's manifest.

### Version 2

- `blobs-v2`: Base64-encoded JSON string containing `sequence`, `validators` and `expiration` fields.
    - `manifest`: OPTIONAL string representing the base-64 or hex-encoded manifest containing the publisher's master and signing public keys.
    - `signature`: string representing the hex-encoded signature of the blob using the publisher's signing key.
    - `blob` string representing the base64-encoded json representation of the blob.
      - `effective` : Unsigned integer representing the ripple time point when the list will become valid.
      - `sequence`: Validator list sequence (incremental)
      - `expiration`: Ripple timestamp (seconds since January 1st, 2000 (00:00 UTC)) for when
          the list expires.
      - `validators` contains an array of objects with a hex `validation_public_key` and a base64-encoded `manifest`
- `manifest`: Base64-encoded serialization of a manifest containing the
        publisher's master and signing public keys.
- `version`: The version of the validator list protocol this object uses. The current version is 1. A higher version number indicates backwards-incompatible changes with a previous version of the validator list protocol.
- `public_key`: The public key used to verify this validator list data, in hexadecimal. This is a 32-byte Ed25519 public key prefixed with the byte 0xED. The value is equal to the `master_public_key` in the publisher's manifest.

### Manifest format

The `manifest` is a data structure that defines a Validator. Manifest are [serialized](https://github.com/elmurci/xrpl-unl-tool/blob/29f30a50a36c2bbcecd642b6f99217dd656e78bc/src/util.rs#L19) and contain the followig fields:

- `sequence`: Manifest sequence number.
- `master_public_key`: The master public key (base58 encoded)
- `signature`: The signature (can be verified with the `signing_public_key`)
- `signing_public_key`: The signing public key (base58 encoded)
- `master_signature`: The signature (can be verified with the `master_public_key`)
- `domain`: Validator domain (optional).

Example:

```
{
  "sequence": 1,
  "master_public_key": "nHBtBkHGfL4NpB54H1AwBaaSJkSJLUSPvnUNAcuNpuffYB51VjH6",
  "signature": "109c8f7ea54617b24305d44af548fade9bdccc10ec43c76e1a4bef3c588817a6c95757244f7a1170b674d36fe2f0531ef2517a07de1df5424aeebb64591bbd0d",
  "signing_public_key": "nHBYNPHW6LJGzHF8AynFg4TdVD9M9wo5YSf7ybgf8Gobu42GHxbd",
  "master_signature": "cb7a643ebf6386ac8fbc1ed3e0dcfc8ff32311a35af6884c2f3b689f1000643a5c07ecd7f1056f43819488078f2c2285fdfa9329f8549127e86e8ccf3a2fdb09",
  "domain": "xrpl.org"
}
```

### Encode (manifest)

Encodes a manifest, example:

```

```

### Decode (manifest)

Decodes a manifest, example:

```

```

### Tests

`cargo tests`