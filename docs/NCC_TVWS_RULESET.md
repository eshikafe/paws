# NCC TVWS Ruleset

## Ruleset Identifier
NCC to provide the `rulesetIds` for TVWS deployment in Nigeria. For now, I have decided to name it `NccTVWS-2019` after the [2019 NCC draft guidelines](https://ncc.gov.ng/media-centre/public-notices/760-public-notice-draft-guidelines-on-the-use-of-television-white-space-tvws-in-nigeria) for TV white space deployment in Nigeria.

## PAWS Initialization

|INIT_REQ                    |  PAWS    |  NCC     |
|:---------------------------|:---------|:---------|
|deviceDesc:DeviceDescriptor | REQUIRED | REQUIRED |
|location:GeoLocation        | REQUIRED | REQUIRED |
|*other:any                  | OPTIONAL |  TBD     |

Notes: TBD means "to be decided by NCC"

|DeviceDescriptor         |  PAWS    |  NCC   |
|:------------------------|:---------|:-------|
|serialNumber:string      | OPTIONAL | TBD    |
|manufacturerId:string    | OPTIONAL | TBD    |
|modelId:string           | OPTIONAL | TBD    |
|rulesetIds:list<string>  | OPTIONAL | TBD    |
|*other:any               | OPTIONAL | TBD    |



|INIT_RESP                     |  PAWS    |   NCC     |
|:-----------------------------|:---------|:----------|
|rulesetInfos:list<RulesetInfo>| REQUIRED |  REQUIRED |
|databaseChange:DbUpdateSpec   | OPTIONAL |   TBD     |
|*other:any                    | OPTIONAL |   TBD     |

 
## PAWS Device Registration

|REGISTRATION_REQ               |    PAWS   |  NCC      |
|:------------------------------|:----------|:----------|
|deviceDesc:DeviceDescriptor    | REQUIRED  | REQUIRED  |
|location:GeoLocation           | REQUIRED  | REQUIRED  |
|deviceOwner:DeviceOwner        | OPTIONAL  |  TBD      |
|antenna:AntennaCharacteristics | OPTIONAL  |  TBD      |
|*other:any                     | OPTIONAL  |  TBD      |

Notes: 

|REGISTRATION_RESP             |  PAWS    |  NCC      |
|:-----------------------------|:---------|:----------|
|rulesetInfos:list<RulesetInfo>| REQUIRED | REQUIRED  |
|databaseChange:DbUpdateSpec   | OPTIONAL |   TBD     |
|*other:any                    | OPTIONAL |   TBD     |

Notes:
    
