#[non_exhaustive]
pub struct BattlenetRpcErrorCode;

// TODO: See if these need to be implemented as flags enum instead
impl BattlenetRpcErrorCode {
    pub const OK: u32 = 0x00000000;
    pub const INTERNAL: u32 = 0x00000001;
    pub const TIMED_OUT: u32 = 0x00000002;
    pub const DENIED: u32 = 0x00000003;
    pub const NOT_EXISTS: u32 = 0x00000004;
    pub const NOT_STARTED: u32 = 0x00000005;
    pub const IN_PROGRESS: u32 = 0x00000006;
    pub const INVALID_ARGS: u32 = 0x00000007;
    pub const INVALID_SUBSCRIBER: u32 = 0x00000008;
    pub const WAITING_FOR_DEPENDENCY: u32 = 0x00000009;
    pub const NO_AUTH: u32 = 0x0000000a;
    pub const PARENTAL_CONTROL_RESTRICTION: u32 = 0x0000000b;
    pub const NO_GAME_ACCOUNT: u32 = 0x0000000c;
    pub const NOT_IMPLEMENTED: u32 = 0x0000000d;
    pub const OBJECT_REMOVED: u32 = 0x0000000e;
    pub const INVALID_ENTITY_ID: u32 = 0x0000000f;
    pub const INVALID_ENTITY_ACCOUNT_ID: u32 = 0x00000010;
    pub const INVALID_ENTITY_GAME_ACCOUNT_ID: u32 = 0x00000011;
    pub const INVALID_AGENT_ID: u32 = 0x00000013;
    pub const INVALID_TARGET_ID: u32 = 0x00000014;
    pub const MODULE_NOT_LOADED: u32 = 0x00000015;
    pub const MODULE_NO_ENTRY_POINT: u32 = 0x00000016;
    pub const MODULE_SIGNATURE_INCORRECT: u32 = 0x00000017;
    pub const MODULE_CREATE_FAILED: u32 = 0x00000018;
    pub const NO_PROGRAM: u32 = 0x00000019;
    pub const API_NOT_READY: u32 = 0x0000001b;
    pub const BAD_VERSION: u32 = 0x0000001c;
    pub const ATTRIBUTE_TOO_MANY_ATTRIBUTES_SET: u32 = 0x0000001d;
    pub const ATTRIBUTE_MAX_SIZE_EXCEEDED: u32 = 0x0000001e;
    pub const ATTRIBUTE_QUOTA_EXCEEDED: u32 = 0x0000001f;
    pub const SERVER_POOL_SERVER_DISAPPEARED: u32 = 0x00000020;
    pub const SERVER_IS_PRIVATE: u32 = 0x00000021;
    pub const DISABLED: u32 = 0x00000022;
    pub const MODULE_NOT_FOUND: u32 = 0x00000024;
    pub const SERVER_BUSY: u32 = 0x00000025;
    pub const NO_BATTLETAG: u32 = 0x00000026;
    pub const INCOMPLETE_PROFANITY_FILTERS: u32 = 0x00000027;
    pub const INVALID_REGION: u32 = 0x00000028;
    pub const EXISTS_ALREADY: u32 = 0x00000029;
    pub const INVALID_SERVER_THUMBPRINT: u32 = 0x0000002a;
    pub const PHONE_LOCK: u32 = 0x0000002b;
    pub const SQUELCHED: u32 = 0x0000002c;
    pub const TARGET_OFFLINE: u32 = 0x0000002d;
    pub const BAD_SERVER: u32 = 0x0000002e;
    pub const NO_COOKIE: u32 = 0x0000002f;
    pub const EXPIRED_COOKIE: u32 = 0x00000030;
    pub const TOKEN_NOT_FOUND: u32 = 0x00000031;
    pub const GAME_ACCOUNT_NO_TIME: u32 = 0x00000032;
    pub const GAME_ACCOUNT_NO_PLAN: u32 = 0x00000033;
    pub const GAME_ACCOUNT_BANNED: u32 = 0x00000034;
    pub const GAME_ACCOUNT_SUSPENDED: u32 = 0x00000035;
    pub const GAME_ACCOUNT_ALREADY_SELECTED: u32 = 0x00000036;
    pub const GAME_ACCOUNT_CANCELLED: u32 = 0x00000037;
    pub const GAME_ACCOUNT_CREATION_DISABLED: u32 = 0x00000038;
    pub const GAME_ACCOUNT_LOCKED: u32 = 0x00000039;

    pub const SESSION_DUPLICATE: u32 = 0x0000003c;
    pub const SESSION_DISCONNECTED: u32 = 0x0000003d;
    pub const SESSION_DATA_CHANGED: u32 = 0x0000003e;
    pub const SESSION_UPDATE_FAILED: u32 = 0x0000003f;
    pub const SESSION_NOT_FOUND: u32 = 0x00000040;

    pub const ADMIN_KICK: u32 = 0x00000046;
    pub const UNPLANNED_MAINTENANCE: u32 = 0x00000047;
    pub const PLANNED_MAINTENANCE: u32 = 0x00000048;
    pub const SERVICE_FAILURE_ACCOUNT: u32 = 0x00000049;
    pub const SERVICE_FAILURE_SESSION: u32 = 0x0000004a;
    pub const SERVICE_FAILURE_AUTH: u32 = 0x0000004b;
    pub const SERVICE_FAILURE_RISK: u32 = 0x0000004c;
    pub const BAD_PROGRAM: u32 = 0x0000004d;
    pub const BAD_LOCALE: u32 = 0x0000004e;
    pub const BAD_PLATFORM: u32 = 0x0000004f;
    pub const LOCALE_RESTRICTED_LA: u32 = 0x00000051;
    pub const LOCALE_RESTRICTED_RU: u32 = 0x00000052;
    pub const LOCALE_RESTRICTED_KO: u32 = 0x00000053;
    pub const LOCALE_RESTRICTED_TW: u32 = 0x00000054;
    pub const LOCALE_RESTRICTED: u32 = 0x00000055;
    pub const ACCOUNT_NEEDS_MAINTENANCE: u32 = 0x00000056;
    pub const MODULE_API_ERROR: u32 = 0x00000057;
    pub const MODULE_BAD_CACHE_HANDLE: u32 = 0x00000058;
    pub const MODULE_ALREADY_LOADED: u32 = 0x00000059;
    pub const NETWORK_BLACKLISTED: u32 = 0x0000005a;
    pub const EVENT_PROCESSOR_SLOW: u32 = 0x0000005b;
    pub const SERVER_SHUTTING_DOWN: u32 = 0x0000005c;
    pub const NETWORK_NOT_PRIVILEGED: u32 = 0x0000005d;
    pub const TOO_MANY_OUTSTANDING_REQUESTS: u32 = 0x0000005e;
    pub const NO_ACCOUNT_REGISTERED: u32 = 0x0000005f;
    pub const BATTLENET_ACCOUNT_BANNED: u32 = 0x00000060;

    pub const OK_DEPRECATED: u32 = 0x00000064;
    pub const SERVER_IN_MODE_ZOMBIE: u32 = 0x00000065;

    pub const LOGON_MODULE_REQUIRED: u32 = 0x000001f4;
    pub const LOGON_MODULE_NOT_CONFIGURED: u32 = 0x000001f5;
    pub const LOGON_MODULE_TIMEOUT: u32 = 0x000001f6;
    pub const LOGON_AGREEMENT_REQUIRED: u32 = 0x000001fe;
    pub const LOGON_AGREEMENT_NOT_CONFIGURED: u32 = 0x000001ff;

    pub const LOGON_INVALID_SERVER_PROOF: u32 = 0x00000208;
    pub const LOGON_WEB_VERIFY_TIMEOUT: u32 = 0x00000209;
    pub const LOGON_INVALID_AUTH_TOKEN: u32 = 0x0000020a;

    pub const CHALLENGE_SMS_TOO_SOON: u32 = 0x00000258;
    pub const CHALLENGE_SMS_THROTTLED: u32 = 0x00000259;
    pub const CHALLENGE_SMS_TEMP_OUTAGE: u32 = 0x0000025a;
    pub const CHALLENGE_NO_CHALLENGE: u32 = 0x0000025b;
    pub const CHALLENGE_NOT_PICKED: u32 = 0x0000025c;
    pub const CHALLENGE_ALREADY_PICKED: u32 = 0x0000025d;
    pub const CHALLENGE_IN_PROGRESS: u32 = 0x0000025e;

    pub const CONFIG_FORMAT_INVALID: u32 = 0x000002bc;
    pub const CONFIG_NOT_FOUND: u32 = 0x000002bd;
    pub const CONFIG_RETRIEVE_FAILED: u32 = 0x000002be;

    pub const NETWORK_MODULE_BUSY: u32 = 0x000003e8;
    pub const NETWORK_MODULE_CANT_RESOLVE_ADDRESS: u32 = 0x000003e9;
    pub const NETWORK_MODULE_CONNECTION_REFUSED: u32 = 0x000003ea;
    pub const NETWORK_MODULE_INTERRUPTED: u32 = 0x000003eb;
    pub const NETWORK_MODULE_CONNECTION_ABORTED: u32 = 0x000003ec;
    pub const NETWORK_MODULE_CONNECTION_RESET: u32 = 0x000003ed;
    pub const NETWORK_MODULE_BAD_ADDRESS: u32 = 0x000003ee;
    pub const NETWORK_MODULE_NOT_READY: u32 = 0x000003ef;
    pub const NETWORK_MODULE_ALREADY_CONNECTED: u32 = 0x000003f0;
    pub const NETWORK_MODULE_CANT_CREATE_SOCKET: u32 = 0x000003f1;
    pub const NETWORK_MODULE_NETWORK_UNREACHABLE: u32 = 0x000003f2;
    pub const NETWORK_MODULE_SOCKET_PERMISSION_DENIED: u32 = 0x000003f3;
    pub const NETWORK_MODULE_NOT_INITIALIZED: u32 = 0x000003f4;
    pub const NETWORK_MODULE_NO_SSL_CERTIFICATE_FOR_PEER: u32 = 0x000003f5;
    pub const NETWORK_MODULE_NO_SSL_COMMON_NAME_FOR_CERTIFICATE: u32 = 0x000003f6;
    pub const NETWORK_MODULE_SSL_COMMON_NAME_DOES_NOT_MATCH_REMOTE_ENDPOINT: u32 = 0x000003f7;
    pub const NETWORK_MODULE_SOCKET_CLOSED: u32 = 0x000003f8;
    pub const NETWORK_MODULE_SSL_PEER_IS_NOT_REGISTERED_IN_CERTBUNDLE: u32 = 0x000003f9;
    pub const NETWORK_MODULE_SSL_INITIALIZE_LOW_FIRST: u32 = 0x000003fa;
    pub const NETWORK_MODULE_SSL_CERT_BUNDLE_READ_ERROR: u32 = 0x000003fb;
    pub const NETWORK_MODULE_NO_CERT_BUNDLE: u32 = 0x000003fc;
    pub const NETWORK_MODULE_FAILED_TO_DOWNLOAD_CERT_BUNDLE: u32 = 0x000003fd;
    pub const NETWORK_MODULE_NOT_READY_TO_READ: u32 = 0x000003fe;

    pub const NETWORK_MODULE_OPENSSL_X509_OK: u32 = 0x000004b0;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_GET_ISSUER_CERT: u32 = 0x000004b1;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_GET_CRL: u32 = 0x000004b2;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_DECRYPT_CERT_SIGNATURE: u32 = 0x000004b3;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_DECRYPT_CRL_SIGNATURE: u32 = 0x000004b4;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_DECODE_ISSUER_PUBLIC_KEY: u32 = 0x000004b5;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_SIGNATURE_FAILURE: u32 = 0x000004b6;
    pub const NETWORK_MODULE_OPENSSL_X509_CRL_SIGNATURE_FAILURE: u32 = 0x000004b7;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_NOT_YET_VALID: u32 = 0x000004b8;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_HAS_EXPIRED: u32 = 0x000004b9;
    pub const NETWORK_MODULE_OPENSSL_X509_CRL_NOT_YET_VALID: u32 = 0x000004ba;
    pub const NETWORK_MODULE_OPENSSL_X509_CRL_HAS_EXPIRED: u32 = 0x000004bb;
    pub const NETWORK_MODULE_OPENSSL_X509_IN_CERT_NOT_BEFORE_FIELD: u32 = 0x000004bc;
    pub const NETWORK_MODULE_OPENSSL_X509_IN_CERT_NOT_AFTER_FIELD: u32 = 0x000004bd;
    pub const NETWORK_MODULE_OPENSSL_X509_IN_CRL_LAST_UPDATE_FIELD: u32 = 0x000004be;
    pub const NETWORK_MODULE_OPENSSL_X509_IN_CRL_NEXT_UPDATE_FIELD: u32 = 0x000004bf;
    pub const NETWORK_MODULE_OPENSSL_X509_OUT_OF_MEM: u32 = 0x000004c0;
    pub const NETWORK_MODULE_OPENSSL_X509_DEPTH_ZERO_SELF_SIGNED_CERT: u32 = 0x000004c1;
    pub const NETWORK_MODULE_OPENSSL_X509_SELF_SIGNED_CERT_IN_CHAIN: u32 = 0x000004c2;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_GET_ISSUER_CERT_LOCALLY: u32 = 0x000004c3;
    pub const NETWORK_MODULE_OPENSSL_X509_UNABLE_TO_VERIFY_LEAF_SIGNATURE: u32 = 0x000004c4;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_CHAIN_TOO_LONG: u32 = 0x000004c5;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_REVOKED: u32 = 0x000004c6;
    pub const NETWORK_MODULE_OPENSSL_X509_INVALID_CA: u32 = 0x000004c7;
    pub const NETWORK_MODULE_OPENSSL_X509_PATH_LENGTH_EXCEEDED: u32 = 0x000004c8;
    pub const NETWORK_MODULE_OPENSSL_X509_INVALID_PURPOSE: u32 = 0x000004c9;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_UNTRUSTED: u32 = 0x000004ca;
    pub const NETWORK_MODULE_OPENSSL_X509_CERT_REJECTED: u32 = 0x000004cb;
    pub const NETWORK_MODULE_OPENSSL_X509_SUBJECT_ISSUER_MISMATCH: u32 = 0x000004cc;
    pub const NETWORK_MODULE_OPENSSL_X509_AKID_SKID_MISMATCH: u32 = 0x000004cd;
    pub const NETWORK_MODULE_OPENSSL_X509_AKID_ISSUER_SERIAL_MISMATCH: u32 = 0x000004ce;
    pub const NETWORK_MODULE_OPENSSL_X509_KEYUSAGE_NO_CERTSIGN: u32 = 0x000004cf;
    pub const NETWORK_MODULE_OPENSSL_X509_APPLICATION_VERIFICATION: u32 = 0x000004d0;

    pub const NETWORK_MODULE_SCHANNEL_CANNOT_FIND_OS_VERSION: u32 = 0x00000514;
    pub const NETWORK_MODULE_SCHANNEL_OS_NOT_SUPPORTED: u32 = 0x00000515;
    pub const NETWORK_MODULE_SCHANNEL_LOADLIBRARY_FAIL: u32 = 0x00000516;
    pub const NETWORK_MODULE_SCHANNEL_CANNOT_FIND_INTERFACE: u32 = 0x00000517;
    pub const NETWORK_MODULE_SCHANNEL_INIT_FAIL: u32 = 0x00000518;
    pub const NETWORK_MODULE_SCHANNEL_FUNCTION_CALL_FAIL: u32 = 0x00000519;
    pub const NETWORK_MODULE_SCHANNEL_X509_UNABLE_TO_GET_ISSUER_CERT: u32 = 0x00000546;
    pub const NETWORK_MODULE_SCHANNEL_X509_TIME_INVALID: u32 = 0x00000547;
    pub const NETWORK_MODULE_SCHANNEL_X509_SIGNATURE_INVALID: u32 = 0x00000548;
    pub const NETWORK_MODULE_SCHANNEL_X509_UNABLE_TO_VERIFY_LEAF_SIGNATURE: u32 = 0x00000549;
    pub const NETWORK_MODULE_SCHANNEL_X509_SELF_SIGNED_LEAF_CERTIFICATE: u32 = 0x0000054a;
    pub const NETWORK_MODULE_SCHANNEL_X509_UNHANDLED_ERROR: u32 = 0x0000054b;
    pub const NETWORK_MODULE_SCHANNEL_X509_SELF_SIGNED_CERT_IN_CHAIN: u32 = 0x0000054c;

    pub const WEBSOCKET_HANDSHAKE: u32 = 0x00000578;

    pub const NETWORK_MODULE_DURANGO_UNKNOWN: u32 = 0x000005dc;
    pub const NETWORK_MODULE_DURANGO_MALFORMED_HOST_NAME: u32 = 0x000005dd;
    pub const NETWORK_MODULE_DURANGO_INVALID_CONNECTION_RESPONSE: u32 = 0x000005de;
    pub const NETWORK_MODULE_DURANGO_INVALID_CA_CERT: u32 = 0x000005df;

    pub const RPC_WRITE_FAILED: u32 = 0x00000bb8;
    pub const RPC_SERVICE_NOT_BOUND: u32 = 0x00000bb9;
    pub const RPC_TOO_MANY_REQUESTS: u32 = 0x00000bba;
    pub const RPC_PEER_UNKNOWN: u32 = 0x00000bbb;
    pub const RPC_PEER_UNAVAILABLE: u32 = 0x00000bbc;
    pub const RPC_PEER_DISCONNECTED: u32 = 0x00000bbd;
    pub const RPC_REQUEST_TIMED_OUT: u32 = 0x00000bbe;
    pub const RPC_CONNECTION_TIMED_OUT: u32 = 0x00000bbf;
    pub const RPC_MALFORMED_RESPONSE: u32 = 0x00000bc0;
    pub const RPC_ACCESS_DENIED: u32 = 0x00000bc1;
    pub const RPC_INVALID_SERVICE: u32 = 0x00000bc2;
    pub const RPC_INVALID_METHOD: u32 = 0x00000bc3;
    pub const RPC_INVALID_OBJECT: u32 = 0x00000bc4;
    pub const RPC_MALFORMED_REQUEST: u32 = 0x00000bc5;
    pub const RPC_QUOTA_EXCEEDED: u32 = 0x00000bc6;
    pub const RPC_NOT_IMPLEMENTED: u32 = 0x00000bc7;
    pub const RPC_SERVER_ERROR: u32 = 0x00000bc8;
    pub const RPC_SHUTDOWN: u32 = 0x00000bc9;
    pub const RPC_DISCONNECT: u32 = 0x00000bca;
    pub const RPC_DISCONNECT_IDLE: u32 = 0x00000bcb;
    pub const RPC_PROTOCOL_ERROR: u32 = 0x00000bcc;
    pub const RPC_NOT_READY: u32 = 0x00000bcd;
    pub const RPC_FORWARD_FAILED: u32 = 0x00000bce;
    pub const RPC_ENCRYPTION_FAILED: u32 = 0x00000bcf;
    pub const RPC_INVALID_ADDRESS: u32 = 0x00000bd0;
    pub const RPC_METHOD_DISABLED: u32 = 0x00000bd1;
    pub const RPC_SHARD_NOT_FOUND: u32 = 0x00000bd2;
    pub const RPC_INVALID_CONNECTION_ID: u32 = 0x00000bd3;
    pub const RPC_NOT_CONNECTED: u32 = 0x00000bd4;
    pub const RPC_INVALID_CONNECTION_STATE: u32 = 0x00000bd5;
    pub const RPC_SERVICE_ALREADY_REGISTERED: u32 = 0x00000bd6;

    pub const PRESENCE_INVALID_FIELD_ID: u32 = 0x00000fa0;
    pub const PRESENCE_NO_VALID_SUBSCRIBERS: u32 = 0x00000fa1;
    pub const PRESENCE_ALREADY_SUBSCRIBED: u32 = 0x00000fa2;
    pub const PRESENCE_CONSUMER_NOT_FOUND: u32 = 0x00000fa3;
    pub const PRESENCE_CONSUMER_IS_NULL: u32 = 0x00000fa4;
    pub const PRESENCE_TEMPORARY_OUTAGE: u32 = 0x00000fa5;
    pub const PRESENCE_TOO_MANY_SUBSCRIPTIONS: u32 = 0x00000fa6;
    pub const PRESENCE_SUBSCRIPTION_CANCELLED: u32 = 0x00000fa7;
    pub const PRESENCE_RICH_PRESENCE_PARSE_ERROR: u32 = 0x00000fa8;
    pub const PRESENCE_RICH_PRESENCE_XML_ERROR: u32 = 0x00000fa9;
    pub const PRESENCE_RICH_PRESENCE_LOAD_ERROR: u32 = 0x00000faa;

    pub const FRIENDS_TOO_MANY_SENT_INVITATIONS: u32 = 0x00001389;
    pub const FRIENDS_TOO_MANY_RECEIVED_INVITATIONS: u32 = 0x0000138a;
    pub const FRIENDS_FRIENDSHIP_ALREADY_EXISTS: u32 = 0x0000138b;
    pub const FRIENDS_FRIENDSHIP_DOES_NOT_EXIST: u32 = 0x0000138c;
    pub const FRIENDS_INVITATION_ALREADY_EXISTS: u32 = 0x0000138d;
    pub const FRIENDS_INVALID_INVITATION: u32 = 0x0000138e;
    pub const FRIENDS_ALREADY_SUBSCRIBED: u32 = 0x0000138f;
    pub const FRIENDS_ACCOUNT_BLOCKED: u32 = 0x00001391;
    pub const FRIENDS_NOT_SUBSCRIBED: u32 = 0x00001392;
    pub const FRIENDS_INVALID_ROLE_ID: u32 = 0x00001393;
    pub const FRIENDS_DISABLED_ROLE_ID: u32 = 0x00001394;
    pub const FRIENDS_NOTE_MAX_SIZE_EXCEEDED: u32 = 0x00001395;
    pub const FRIENDS_UPDATE_FRIEND_STATE_FAILED: u32 = 0x00001396;
    pub const FRIENDS_INVITEE_AT_MAX_FRIENDS: u32 = 0x00001397;
    pub const FRIENDS_INVITER_AT_MAX_FRIENDS: u32 = 0x00001398;

    pub const PLATFORM_STORAGE_FILE_WRITE_DENIED: u32 = 0x00001770;

    pub const WHISPER_UNDELIVERABLE: u32 = 0x00001b58;
    pub const WHISPER_MAX_SIZE_EXCEEDED: u32 = 0x00001b59;

    pub const USER_MANAGER_ALREADY_BLOCKED: u32 = 0x00001f40;
    pub const USER_MANAGER_NOT_BLOCKED: u32 = 0x00001f41;
    pub const USER_MANAGER_CANNOT_BLOCK_SELF: u32 = 0x00001f42;
    pub const USER_MANAGER_ALREADY_REGISTERED: u32 = 0x00001f43;
    pub const USER_MANAGER_NOT_REGISTERED: u32 = 0x00001f44;
    pub const USER_MANAGER_TOO_MANY_BLOCKED_ENTITIES: u32 = 0x00001f45;
    pub const USER_MANAGER_TOO_MANY_IDS: u32 = 0x00001f47;
    pub const USER_MANAGER_BLOCK_RECORD_UNAVAILABLE: u32 = 0x00001f4f;
    pub const USER_MANAGER_BLOCK_ENTITY_FAILED: u32 = 0x00001f50;
    pub const USER_MANAGER_UNBLOCK_ENTITY_FAILED: u32 = 0x00001f51;
    pub const USER_MANAGER_CANNOT_BLOCK_FRIEND: u32 = 0x00001f53;

    pub const SOCIAL_NETWORK_DB_EXCEPTION: u32 = 0x00002328;
    pub const SOCIAL_NETWORK_DENIAL_FROM_PROVIDER: u32 = 0x00002329;
    pub const SOCIAL_NETWORK_INVALID_SNS_ID: u32 = 0x0000232a;
    pub const SOCIAL_NETWORK_CANT_SEND_TO_PROVIDER: u32 = 0x0000232b;
    pub const SOCIAL_NETWORK_EX_COMM_FAILED: u32 = 0x0000232c;
    pub const SOCIAL_NETWORK_DISABLED: u32 = 0x0000232d;
    pub const SOCIAL_NETWORK_MISSING_REQUEST_PARAM: u32 = 0x0000232e;
    pub const SOCIAL_NETWORK_UNSUPPORTED_OAUTH_VERSION: u32 = 0x0000232f;

    pub const CHANNEL_FULL: u32 = 0x00002710;
    pub const CHANNEL_NO_CHANNEL: u32 = 0x00002711;
    pub const CHANNEL_NOT_MEMBER: u32 = 0x00002712;
    pub const CHANNEL_ALREADY_MEMBER: u32 = 0x00002713;
    pub const CHANNEL_NO_SUCH_MEMBER: u32 = 0x00002714;
    pub const CHANNEL_INVALID_CHANNEL_ID: u32 = 0x00002716;
    pub const CHANNEL_NO_SUCH_INVITATION: u32 = 0x00002718;
    pub const CHANNEL_TOO_MANY_INVITATIONS: u32 = 0x00002719;
    pub const CHANNEL_INVITATION_ALREADY_EXISTS: u32 = 0x0000271a;
    pub const CHANNEL_INVALID_CHANNEL_SIZE: u32 = 0x0000271b;
    pub const CHANNEL_INVALID_ROLE_ID: u32 = 0x0000271c;
    pub const CHANNEL_ROLE_NOT_ASSIGNABLE: u32 = 0x0000271d;
    pub const CHANNEL_INSUFFICIENT_PRIVILEGES: u32 = 0x0000271e;
    pub const CHANNEL_INSUFFICIENT_PRIVACY_LEVEL: u32 = 0x0000271f;
    pub const CHANNEL_INVALID_PRIVACY_LEVEL: u32 = 0x00002720;
    pub const CHANNEL_TOO_MANY_CHANNELS_JOINED: u32 = 0x00002721;
    pub const CHANNEL_INVITATION_ALREADY_SUBSCRIBED: u32 = 0x00002722;
    pub const CHANNEL_INVALID_CHANNEL_DELEGATE: u32 = 0x00002723;
    pub const CHANNEL_SLOT_ALREADY_RESERVED: u32 = 0x00002724;
    pub const CHANNEL_SLOT_NOT_RESERVED: u32 = 0x00002725;
    pub const CHANNEL_NO_RESERVED_SLOTS_AVAILABLE: u32 = 0x00002726;
    pub const CHANNEL_INVALID_ROLE_SET: u32 = 0x00002727;
    pub const CHANNEL_REQUIRE_FRIEND_VALIDATION: u32 = 0x00002728;
    pub const CHANNEL_MEMBER_OFFLINE: u32 = 0x00002729;
    pub const CHANNEL_RECEIVED_TOO_MANY_INVITATIONS: u32 = 0x0000272a;
    pub const CHANNEL_INVITATION_INVALID_GAME_ACCOUNT_SELECTED: u32 = 0x0000272b;
    pub const CHANNEL_UNREACHABLE: u32 = 0x0000272c;
    pub const CHANNEL_INVITATION_NOT_SUBSCRIBED: u32 = 0x0000272d;
    pub const CHANNEL_INVALID_MESSAGE_SIZE: u32 = 0x0000272e;
    pub const CHANNEL_MAX_MESSAGE_SIZE_EXCEEDED: u32 = 0x0000272f;
    pub const CHANNEL_CONFIG_NOT_FOUND: u32 = 0x00002730;
    pub const CHANNEL_INVALID_CHANNEL_TYPE: u32 = 0x00002731;

    pub const LOCAL_STORAGE_FILE_OPEN_ERROR: u32 = 0x00002af8;
    pub const LOCAL_STORAGE_FILE_CREATE_ERROR: u32 = 0x00002af9;
    pub const LOCAL_STORAGE_FILE_READ_ERROR: u32 = 0x00002afa;
    pub const LOCAL_STORAGE_FILE_WRITE_ERROR: u32 = 0x00002afb;
    pub const LOCAL_STORAGE_FILE_DELETE_ERROR: u32 = 0x00002afc;
    pub const LOCAL_STORAGE_FILE_COPY_ERROR: u32 = 0x00002afd;
    pub const LOCAL_STORAGE_FILE_DECOMPRESS_ERROR: u32 = 0x00002afe;
    pub const LOCAL_STORAGE_FILE_HASH_MISMATCH: u32 = 0x00002aff;
    pub const LOCAL_STORAGE_FILE_USAGE_MISMATCH: u32 = 0x00002b00;
    pub const LOCAL_STORAGE_DATABASE_INIT_ERROR: u32 = 0x00002b01;
    pub const LOCAL_STORAGE_DATABASE_NEEDS_REBUILD: u32 = 0x00002b02;
    pub const LOCAL_STORAGE_DATABASE_INSERT_ERROR: u32 = 0x00002b03;
    pub const LOCAL_STORAGE_DATABASE_LOOKUP_ERROR: u32 = 0x00002b04;
    pub const LOCAL_STORAGE_DATABASE_UPDATE_ERROR: u32 = 0x00002b05;
    pub const LOCAL_STORAGE_DATABASE_DELETE_ERROR: u32 = 0x00002b06;
    pub const LOCAL_STORAGE_DATABASE_SHRINK_ERROR: u32 = 0x00002b07;
    pub const LOCAL_STORAGE_CACHE_CRAWL_ERROR: u32 = 0x00002b08;
    pub const LOCAL_STORAGE_DATABASE_INDEX_TRIGGER_ERROR: u32 = 0x00002b09;
    pub const LOCAL_STORAGE_DATABASE_REBUILD_IN_PROGRESS: u32 = 0x00002b0a;
    pub const LOCAL_STORAGE_OK_BUT_NOT_IN_CACHE: u32 = 0x00002b0b;
    pub const LOCAL_STORAGE_DATABASE_REBUILD_INTERRUPTED: u32 = 0x00002b0d;
    pub const LOCAL_STORAGE_DATABASE_NOT_INITIALIZED: u32 = 0x00002b0e;
    pub const LOCAL_STORAGE_DIRECTORY_CREATE_ERROR: u32 = 0x00002b0f;
    pub const LOCAL_STORAGE_FILEKEY_NOT_FOUND: u32 = 0x00002b10;
    pub const LOCAL_STORAGE_NOT_AVAILABLE_ON_SERVER: u32 = 0x00002b11;

    pub const REGISTRY_CREATE_KEY_ERROR: u32 = 0x00002ee0;
    pub const REGISTRY_OPEN_KEY_ERROR: u32 = 0x00002ee1;
    pub const REGISTRY_READ_ERROR: u32 = 0x00002ee2;
    pub const REGISTRY_WRITE_ERROR: u32 = 0x00002ee3;
    pub const REGISTRY_TYPE_ERROR: u32 = 0x00002ee4;
    pub const REGISTRY_DELETE_ERROR: u32 = 0x00002ee5;
    pub const REGISTRY_ENCRYPT_ERROR: u32 = 0x00002ee6;
    pub const REGISTRY_DECRYPT_ERROR: u32 = 0x00002ee7;
    pub const REGISTRY_KEY_SIZE_ERROR: u32 = 0x00002ee8;
    pub const REGISTRY_VALUE_SIZE_ERROR: u32 = 0x00002ee9;
    pub const REGISTRY_NOT_FOUND: u32 = 0x00002eeb;
    pub const REGISTRY_MALFORMED_STRING: u32 = 0x00002eec;

    pub const INTERFACE_ALREADY_CONNECTED: u32 = 0x000032c8;
    pub const INTERFACE_NOT_READY: u32 = 0x000032c9;
    pub const INTERFACE_OPTION_KEY_TOO_LARGE: u32 = 0x000032ca;
    pub const INTERFACE_OPTION_VALUE_TOO_LARGE: u32 = 0x000032cb;
    pub const INTERFACE_OPTION_KEY_INVALID_UTF8_STRING: u32 = 0x000032cc;
    pub const INTERFACE_OPTION_VALUE_INVALID_UTF8_STRING: u32 = 0x000032cd;

    pub const HTTP_COULDNT_RESOLVE: u32 = 0x000036b0;
    pub const HTTP_COULDNT_CONNECT: u32 = 0x000036b1;
    pub const HTTP_TIMEOUT: u32 = 0x000036b2;
    pub const HTTP_FAILED: u32 = 0x000036b3;
    pub const HTTP_MALFORMED_URL: u32 = 0x000036b4;
    pub const HTTP_DOWNLOAD_ABORTED: u32 = 0x000036b5;
    pub const HTTP_COULDNT_WRITE_FILE: u32 = 0x000036b6;
    pub const HTTP_TOO_MANY_REDIRECTS: u32 = 0x000036b7;
    pub const HTTP_COULDNT_OPEN_FILE: u32 = 0x000036b8;
    pub const HTTP_COULDNT_CREATE_FILE: u32 = 0x000036b9;
    pub const HTTP_COULDNT_READ_FILE: u32 = 0x000036ba;
    pub const HTTP_COULDNT_RENAME_FILE: u32 = 0x000036bb;
    pub const HTTP_COULDNT_CREATE_DIRECTORY: u32 = 0x000036bc;
    pub const HTTP_CURL_IS_NOT_READY: u32 = 0x000036bd;
    pub const HTTP_CANCELLED: u32 = 0x000036be;

    pub const HTTP_FILE_NOT_FOUND: u32 = 0x00003844;

    pub const ACCOUNT_MISSING_CONFIG: u32 = 0x00004650;
    pub const ACCOUNT_DATA_NOT_FOUND: u32 = 0x00004651;
    pub const ACCOUNT_ALREADY_SUBSCRIBED: u32 = 0x00004652;
    pub const ACCOUNT_NOT_SUBSCRIBED: u32 = 0x00004653;
    pub const ACCOUNT_FAILED_TO_PARSE_TIMEZONE_DATA: u32 = 0x00004654;
    pub const ACCOUNT_LOAD_FAILED: u32 = 0x00004655;
    pub const ACCOUNT_LOAD_CANCELLED: u32 = 0x00004656;
    pub const ACCOUNT_DATABASE_INVALIDATE_FAILED: u32 = 0x00004657;
    pub const ACCOUNT_CACHE_INVALIDATE_FAILED: u32 = 0x00004658;
    pub const ACCOUNT_SUBSCRIPTION_PENDING: u32 = 0x00004659;
    pub const ACCOUNT_UNKNOWN_REGION: u32 = 0x0000465a;
    pub const ACCOUNT_DATA_FAILED_TO_PARSE: u32 = 0x0000465b;
    pub const ACCOUNT_UNDERAGE: u32 = 0x0000465c;
    pub const ACCOUNT_IDENTITY_CHECK_PENDING: u32 = 0x0000465d;
    pub const ACCOUNT_IDENTITY_UNVERIFIED: u32 = 0x0000465e;

    pub const DATABASE_BINDING_COUNT_MISMATCH: u32 = 0x00004a38;
    pub const DATABASE_BINDING_PARSE_FAIL: u32 = 0x00004a39;
    pub const DATABASE_RESULTSET_COLUMNS_MISMATCH: u32 = 0x00004a3a;
    pub const DATABASE_DEADLOCK: u32 = 0x00004a3b;
    pub const DATABASE_DUPLICATE_KEY: u32 = 0x00004a3c;
    pub const DATABASE_CANNOT_CONNECT: u32 = 0x00004a3d;
    pub const DATABASE_STATEMENT_FAILED: u32 = 0x00004a3e;
    pub const DATABASE_TRANSACTION_NOT_STARTED: u32 = 0x00004a3f;
    pub const DATABASE_TRANSACTION_NOT_ENDED: u32 = 0x00004a40;
    pub const DATABASE_TRANSACTION_LEAK: u32 = 0x00004a41;
    pub const DATABASE_TRANSACTION_STATE_BAD: u32 = 0x00004a42;
    pub const DATABASE_SERVER_GONE: u32 = 0x00004a43;
    pub const DATABASE_QUERY_TIMEOUT: u32 = 0x00004a44;
    pub const DATABASE_BINDING_NOT_NULLABLE: u32 = 0x00004a9c;
    pub const DATABASE_BINDING_INVALID_INTEGER: u32 = 0x00004a9d;
    pub const DATABASE_BINDING_INVALID_FLOAT: u32 = 0x00004a9e;
    pub const DATABASE_BINDING_INVALID_TEMPORAL: u32 = 0x00004a9f;
    pub const DATABASE_BINDING_INVALID_PROTOBUF: u32 = 0x00004aa0;

    pub const PARTY_INVALID_PARTY_ID: u32 = 0x00004e20;
    pub const PARTY_ALREADY_IN_PARTY: u32 = 0x00004e21;
    pub const PARTY_NOT_IN_PARTY: u32 = 0x00004e22;
    pub const PARTY_INVITATION_UNDELIVERABLE: u32 = 0x00004e23;
    pub const PARTY_INVITATION_ALREADY_EXISTS: u32 = 0x00004e24;
    pub const PARTY_TOO_MANY_PARTY_INVITATIONS: u32 = 0x00004e25;
    pub const PARTY_TOO_MANY_RECEIVED_INVITATIONS: u32 = 0x00004e26;
    pub const PARTY_NO_SUCH_TYPE: u32 = 0x00004e27;

    pub const GAMES_NO_SUCH_FACTORY: u32 = 0x000055f0;
    pub const GAMES_NO_SUCH_GAME: u32 = 0x000055f1;
    pub const GAMES_NO_SUCH_REQUEST: u32 = 0x000055f2;
    pub const GAMES_NO_SUCH_PARTY_MEMBER: u32 = 0x000055f3;

    pub const RESOURCES_OFFLINE: u32 = 0x000059d8;

    pub const GAME_SERVER_CREATE_GAME_REFUSED: u32 = 0x00005dc0;
    pub const GAME_SERVER_ADD_PLAYERS_REFUSED: u32 = 0x00005dc1;
    pub const GAME_SERVER_REMOVE_PLAYERS_REFUSED: u32 = 0x00005dc2;
    pub const GAME_SERVER_FINISH_GAME_REFUSED: u32 = 0x00005dc3;
    pub const GAME_SERVER_NO_SUCH_GAME: u32 = 0x00005dc4;
    pub const GAME_SERVER_NO_SUCH_PLAYER: u32 = 0x00005dc5;
    pub const GAME_SERVER_CREATE_GAME_REFUSED_TRANSIENT: u32 = 0x00005df2;
    pub const GAME_SERVER_ADD_PLAYERS_REFUSED_TRANSIENT: u32 = 0x00005df3;
    pub const GAME_SERVER_REMOVE_PLAYERS_REFUSED_TRANSIENT: u32 = 0x00005df4;
    pub const GAME_SERVER_FINISH_GAME_REFUSED_TRANSIENT: u32 = 0x00005df5;
    pub const GAME_SERVER_CREATE_GAME_REFUSED_BUSY: u32 = 0x00005e24;
    pub const GAME_SERVER_ADD_PLAYERS_REFUSED_BUSY: u32 = 0x00005e25;
    pub const GAME_SERVER_REMOVE_PLAYERS_REFUSED_BUSY: u32 = 0x00005e26;
    pub const GAME_SERVER_FINISH_GAME_REFUSED_BUSY: u32 = 0x00005e27;

    pub const GAME_MASTER_INVALID_FACTORY: u32 = 0x000061a8;
    pub const GAME_MASTER_INVALID_GAME: u32 = 0x000061a9;
    pub const GAME_MASTER_GAME_FULL: u32 = 0x000061aa;
    pub const GAME_MASTER_REGISTER_FAILED: u32 = 0x000061ab;
    pub const GAME_MASTER_NO_GAME_SERVER: u32 = 0x000061ac;
    pub const GAME_MASTER_NO_UTILITY_SERVER: u32 = 0x000061ad;
    pub const GAME_MASTER_NO_GAME_VERSION: u32 = 0x000061ae;
    pub const GAME_MASTER_GAME_JOIN_FAILED: u32 = 0x000061af;
    pub const GAME_MASTER_ALREADY_REGISTERED: u32 = 0x000061b0;
    pub const GAME_MASTER_NO_FACTORY: u32 = 0x000061b1;
    pub const GAME_MASTER_MULTIPLE_GAME_VERSIONS: u32 = 0x000061b2;
    pub const GAME_MASTER_INVALID_PLAYER: u32 = 0x000061b3;
    pub const GAME_MASTER_INVALID_GAME_REQUEST: u32 = 0x000061b4;
    pub const GAME_MASTER_INSUFFICIENT_PRIVILEGES: u32 = 0x000061b5;
    pub const GAME_MASTER_ALREADY_IN_GAME: u32 = 0x000061b6;
    pub const GAME_MASTER_INVALID_GAME_SERVER_RESPONSE: u32 = 0x000061b7;
    pub const GAME_MASTER_GAME_ACCOUNT_LOOKUP_FAILED: u32 = 0x000061b8;
    pub const GAME_MASTER_GAME_ENTRY_CANCELLED: u32 = 0x000061b9;
    pub const GAME_MASTER_GAME_ENTRY_ABORTED_CLIENT_DROPPED: u32 = 0x000061ba;
    pub const GAME_MASTER_GAME_ENTRY_ABORTED_BY_SERVICE: u32 = 0x000061bb;
    pub const GAME_MASTER_NO_AVAILABLE_CAPACITY: u32 = 0x000061bc;
    pub const GAME_MASTER_INVALID_TEAM_ID: u32 = 0x000061bd;
    pub const GAME_MASTER_CREATION_IN_PROGRESS: u32 = 0x000061be;

    pub const NOTIFICATION_INVALID_CLIENT_ID: u32 = 0x00006590;
    pub const NOTIFICATION_DUPLICATE_NAME: u32 = 0x00006591;
    pub const NOTIFICATION_NAME_NOT_FOUND: u32 = 0x00006592;
    pub const NOTIFICATION_INVALID_SERVER: u32 = 0x00006593;
    pub const NOTIFICATION_QUOTA_EXCEEDED: u32 = 0x00006594;
    pub const NOTIFICATION_INVALID_NOTIFICATION_TYPE: u32 = 0x00006595;
    pub const NOTIFICATION_UNDELIVERABLE: u32 = 0x00006596;
    pub const NOTIFICATION_UNDELIVERABLE_TEMPORARY: u32 = 0x00006597;

    pub const ACHIEVEMENTS_NOTHING_TO_UPDATE: u32 = 0x00006d60;
    pub const ACHIEVEMENTS_INVALID_PARAMS: u32 = 0x00006d61;
    pub const ACHIEVEMENTS_NOT_REGISTERED: u32 = 0x00006d62;
    pub const ACHIEVEMENTS_NOT_READY: u32 = 0x00006d63;
    pub const ACHIEVEMENTS_FAILED_TO_PARSE_STATIC_DATA: u32 = 0x00006d64;
    pub const ACHIEVEMENTS_UNKNOWN_ID: u32 = 0x00006d65;
    pub const ACHIEVEMENTS_MISSING_SNAPSHOT: u32 = 0x00006d66;
    pub const ACHIEVEMENTS_ALREADY_REGISTERED: u32 = 0x00006d67;
    pub const ACHIEVEMENTS_TOO_MANY_REGISTRATIONS: u32 = 0x00006d68;
    pub const ACHIEVEMENTS_ALREADY_IN_PROGRESS: u32 = 0x00006d69;
    pub const ACHIEVEMENTS_TEMPORARY_OUTAGE: u32 = 0x00006d6a;
    pub const ACHIEVEMENTS_INVALID_PROGRAMID: u32 = 0x00006d6b;
    pub const ACHIEVEMENTS_MISSING_RECORD: u32 = 0x00006d6c;
    pub const ACHIEVEMENTS_REGISTRATION_PENDING: u32 = 0x00006d6d;
    pub const ACHIEVEMENTS_ENTITY_ID_NOT_FOUND: u32 = 0x00006d6e;
    pub const ACHIEVEMENTS_ACHIEVEMENT_ID_NOT_FOUND: u32 = 0x00006d6f;
    pub const ACHIEVEMENTS_CRITERIA_ID_NOT_FOUND: u32 = 0x00006d70;
    pub const ACHIEVEMENTS_STATIC_DATA_MISMATCH: u32 = 0x00006d71;
    pub const ACHIEVEMENTS_WRONG_THREAD: u32 = 0x00006d72;
    pub const ACHIEVEMENTS_CALLBACK_IS_NULL: u32 = 0x00006d73;
    pub const ACHIEVEMENTS_AUTO_REGISTER_PENDING: u32 = 0x00006d74;
    pub const ACHIEVEMENTS_NOT_INITIALIZED: u32 = 0x00006d75;
    pub const ACHIEVEMENTS_ACHIEVEMENT_ID_ALREADY_EXISTS: u32 = 0x00006d76;
    pub const ACHIEVEMENTS_FAILED_TO_DOWNLOAD_STATIC_DATA: u32 = 0x00006d77;
    pub const ACHIEVEMENTS_STATIC_DATA_NOT_FOUND: u32 = 0x00006d78;

    pub const GAME_UTILITY_SERVER_VARIABLE_REQUEST_REFUSED: u32 = 0x000084d1;
    pub const GAME_UTILITY_SERVER_WRONG_NUMBER_OF_VARIABLES_RETURNED: u32 = 0x000084d2;
    pub const GAME_UTILITY_SERVER_CLIENT_REQUEST_REFUSED: u32 = 0x000084d3;
    pub const GAME_UTILITY_SERVER_PRESENCE_CHANNEL_CREATED_REFUSED: u32 = 0x000084d4;
    pub const GAME_UTILITY_SERVER_VARIABLE_REQUEST_REFUSED_TRANSIENT: u32 = 0x00008502;
    pub const GAME_UTILITY_SERVER_CLIENT_REQUEST_REFUSED_TRANSIENT: u32 = 0x00008503;
    pub const GAME_UTILITY_SERVER_PRESENCE_CHANNEL_CREATED_REFUSED_TRANSIENT: u32 = 0x00008504;
    pub const GAME_UTILITY_SERVER_SERVER_REQUEST_REFUSED_TRANSIENT: u32 = 0x00008505;
    pub const GAME_UTILITY_SERVER_VARIABLE_REQUEST_REFUSED_BUSY: u32 = 0x00008534;
    pub const GAME_UTILITY_SERVER_CLIENT_REQUEST_REFUSED_BUSY: u32 = 0x00008535;
    pub const GAME_UTILITY_SERVER_PRESENCE_CHANNEL_CREATED_REFUSED_BUSY: u32 = 0x00008536;
    pub const GAME_UTILITY_SERVER_SERVER_REQUEST_REFUSED_BUSY: u32 = 0x00008537;
    pub const GAME_UTILITY_SERVER_NO_SERVER: u32 = 0x00008598;

    pub const IDENTITY_INSUFFICIENT_DATA: u32 = 0x0000a028;
    pub const IDENTITY_TOO_MANY_RESULTS: u32 = 0x0000a029;
    pub const IDENTITY_BAD_ID: u32 = 0x0000a02a;
    pub const IDENTITY_NO_ACCOUNT_BLOB: u32 = 0x0000a02b;

    pub const RISK_CHALLENGE_ACTION: u32 = 0x0000a410;
    pub const RISK_DELAY_ACTION: u32 = 0x0000a411;
    pub const RISK_THROTTLE_ACTION: u32 = 0x0000a412;
    pub const RISK_ACCOUNT_LOCKED: u32 = 0x0000a413;
    pub const RISK_CS_DENIED: u32 = 0x0000a414;
    pub const RISK_DISCONNECT_ACCOUNT: u32 = 0x0000a415;
    pub const RISK_CHECK_SKIPPED: u32 = 0x0000a416;

    pub const REPORT_UNAVAILABLE: u32 = 0x0000afc8;
    pub const REPORT_TOO_LARGE: u32 = 0x0000afc9;
    pub const REPORT_UNKNOWN_TYPE: u32 = 0x0000afca;
    pub const REPORT_ATTRIBUTE_INVALID: u32 = 0x0000afcb;
    pub const REPORT_ATTRIBUTE_QUOTA_EXCEEDED: u32 = 0x0000afcc;
    pub const REPORT_UNCONFIRMED: u32 = 0x0000afcd;
    pub const REPORT_NOT_CONNECTED: u32 = 0x0000afce;
    pub const REPORT_REJECTED: u32 = 0x0000afcf;
    pub const REPORT_TOO_MANY_REQUESTS: u32 = 0x0000afd0;

    pub const ACCOUNT_ALREADY_REGISTERD: u32 = 0x0000bb80;
    pub const ACCOUNT_NOT_REGISTERED: u32 = 0x0000bb81;
    pub const ACCOUNT_REGISTRATION_PENDING: u32 = 0x0000bb82;

    pub const MEMCACHED_CLIENT_NO_ERROR: u32 = 0x00010000;
    pub const MEMCACHED_CLIENT_KEY_NOT_FOUND: u32 = 0x00010001;
    pub const MEMCACHED_KEY_EXISTS: u32 = 0x00010002;
    pub const MEMCACHED_VALUE_TO_LARGE: u32 = 0x00010003;
    pub const MEMCACHED_INVALID_ARGS: u32 = 0x00010004;
    pub const MEMCACHED_ITEM_NOT_STORED: u32 = 0x00010005;
    pub const MEMCACHED_NON_NUMERIC_VALUE: u32 = 0x00010006;
    pub const MEMCACHED_WRONG_SERVER: u32 = 0x00010007;
    pub const MEMCACHED_AUTHENTICATION_ERROR: u32 = 0x00010008;
    pub const MEMCACHED_AUTHENTICATION_CONTINUE: u32 = 0x00010009;
    pub const MEMCACHED_UNKNOWN_COMMAND: u32 = 0x0001000a;
    pub const MEMCACHED_OUT_OF_MEMORY: u32 = 0x0001000b;
    pub const MEMCACHED_NOT_SUPPORTED: u32 = 0x0001000c;
    pub const MEMCACHED_INTERNAL_ERROR: u32 = 0x0001000d;
    pub const MEMCACHED_TEMPORARY_FAILURE: u32 = 0x0001000e;

    pub const MEMCACHED_CLIENT_ALREADY_CONNECTED: u32 = 0x000186a0;
    pub const MEMCACHED_CLIENT_BAD_CONFIG: u32 = 0x000186a1;
    pub const MEMCACHED_CLIENT_NOT_CONNECTED: u32 = 0x000186a2;
    pub const MEMCACHED_CLIENT_TIMEOUT: u32 = 0x000186a3;
    pub const MEMCACHED_CLIENT_ABORTED: u32 = 0x000186a4;

    pub const UTIL_SERVER_FAILED_TO_SERIALIZE: u32 = 0x80000064;
    pub const UTIL_SERVER_DISCONNECTED_FROM_BATTLENET: u32 = 0x80000065;
    pub const UTIL_SERVER_TIMED_OUT: u32 = 0x80000066;
    pub const UTIL_SERVER_NO_METERING_DATA: u32 = 0x80000067;
    pub const UTIL_SERVER_FAIL_PERMISSION_CHECK: u32 = 0x80000068;
    pub const UTIL_SERVER_UNKNOWN_REALM: u32 = 0x80000069;
    pub const UTIL_SERVER_MISSING_SESSION_KEY: u32 = 0x8000006a;
    pub const UTIL_SERVER_MISSING_VIRTUAL_REALM: u32 = 0x8000006b;
    pub const UTIL_SERVER_INVALID_SESSION_KEY: u32 = 0x8000006c;
    pub const UTIL_SERVER_MISSING_REALM_LIST: u32 = 0x8000006d;
    pub const UTIL_SERVER_INVALID_IDENTITY_ARGS: u32 = 0x8000006e;
    pub const UTIL_SERVER_SESSION_OBJECT_MISSING: u32 = 0x8000006f;
    pub const UTIL_SERVER_INVALID_BNET_SESSION: u32 = 0x80000070;
    pub const UTIL_SERVER_INVALID_VIRTUAL_REALM: u32 = 0x80000071;
    pub const UTIL_SERVER_INVALID_CLIENT_ADDRESS: u32 = 0x80000072;
    pub const UTIL_SERVER_FAILED_TO_SERIALIZE_RESPONSE: u32 = 0x80000073;
    pub const UTIL_SERVER_UNKNOWN_REQUEST: u32 = 0x80000074;
    pub const UTIL_SERVER_UNABLE_TO_GENERATE_JOIN_TICKET: u32 = 0x80000075;
    pub const UTIL_SERVER_UNABLE_TO_GENERATE_REALM_LIST_TICKET: u32 = 0x80000076;
    pub const UTIL_SERVER_ACCOUNT_DENIED: u32 = 0x80000077;
    pub const UTIL_SERVER_INVALID_WOW_ACCOUNT: u32 = 0x80000078;
    pub const UTIL_SERVER_UNABLE_TO_STORE_SESSION: u32 = 0x80000079;
    pub const UTIL_SERVER_SESSION_ALREADY_CREATED: u32 = 0x8000007a;

    pub const USER_SERVER_FAILED_TO_SERIALIZE: u32 = 0x800000c8;
    pub const USER_SERVER_DISCONNECTED_FROM_UTIL: u32 = 0x800000c9;
    pub const USER_SERVER_SESSION_DUPLICATE: u32 = 0x800000ca;
    pub const USER_SERVER_FAILED_TO_DISABLE_BILLING: u32 = 0x800000cb;
    pub const USER_SERVER_PLAYER_DISCONNECTED: u32 = 0x800000cc;
    pub const USER_SERVER_FAILED_TO_PARSE_ACCOUNT_STATE: u32 = 0x800000cd;
    pub const USER_SERVER_ACCOUNT_LOAD_CANCELLED: u32 = 0x800000ce;
    pub const USER_SERVER_BAD_PLATFORM: u32 = 0x800000cf;
    pub const USER_SERVER_BAD_VIRTUAL_REALM: u32 = 0x800000d0;
    pub const USER_SERVER_LOCALE_RESTRICTED: u32 = 0x800000d1;
    pub const USER_SERVER_MISSING_PROPASS: u32 = 0x800000d2;
    pub const USER_SERVER_BAD_WOW_ACCOUNT: u32 = 0x800000d3;
    pub const USER_SERVER_BAD_BNET_ACCOUNT: u32 = 0x800000d4;
    pub const USER_SERVER_FAILED_TO_PARSE_GAME_ACCOUNT_STATE: u32 = 0x800000d5;
    pub const USER_SERVER_FAILED_TO_PARSE_GAME_TIME_REMAINING: u32 = 0x800000d6;
    pub const USER_SERVER_FAILED_TO_PARSE_GAME_SESSION_INFO: u32 = 0x800000d7;
    pub const USER_SERVER_ACCOUNT_STATE_POORLY_FORMED: u32 = 0x800000d8;
    pub const USER_SERVER_GAME_ACCOUNT_STATE_POORLY_FORMED: u32 = 0x800000d9;
    pub const USER_SERVER_GAME_TIME_REMAINING_POORLY_FORMED: u32 = 0x800000da;
    pub const USER_SERVER_GAME_SESSION_INFO_POORLY_FORMED: u32 = 0x800000db;
    pub const USER_SERVER_BAD_SESSION_TRACKER_STATE: u32 = 0x800000dc;
    pub const USER_SERVER_FAILED_TO_PARSE_CAIS_INFO: u32 = 0x800000dd;
    pub const USER_SERVER_GAME_SESSION_DISCONNECTED: u32 = 0x800000de;
    pub const USER_SERVER_VERSION_MISMATCH: u32 = 0x800000df;
    pub const USER_SERVER_ACCOUNT_SUSPENDED: u32 = 0x800000e0;
    pub const USER_SERVER_NOT_PERMITTED_ON_REALM: u32 = 0x800000e1;
    pub const USER_SERVER_LOGIN_FAILED_CONNECT: u32 = 0x800000e2;

    pub const WOW_SERVICES_TIMED_OUT: u32 = 0x8000012c;
    pub const WOW_SERVICES_INVALID_REALM_LIST_TICKET: u32 = 0x8000012d;
    pub const WOW_SERVICES_INVALID_JOIN_TICKET: u32 = 0x8000012e;
    pub const WOW_SERVICES_INVALID_SERVER_ADDRESSES: u32 = 0x8000012f;
    pub const WOW_SERVICES_INVALID_SECRET_BLOB: u32 = 0x80000130;
    pub const WOW_SERVICES_NO_REALM_JOIN_IP_FOUND: u32 = 0x80000131;
    pub const WOW_SERVICES_DENIED_REALM_LIST_TICKET: u32 = 0x80000132;
    pub const WOW_SERVICES_MISSING_GAME_ACCOUNT: u32 = 0x80000133;
    pub const WOW_SERVICES_LOGON_INVALID_AUTH_TOKEN: u32 = 0x80000134;
    pub const WOW_SERVICES_NO_AVAILABLE_REALMS: u32 = 0x80000135;
    pub const WOW_SERVICES_FAILED_TO_PARSE_DISPATCH: u32 = 0x80000136;
    pub const WOW_SERVICES_MISSING_METERING_FILE: u32 = 0x80000137;
    pub const WOW_SERVICES_LOGIN_INVALID_CONTENT_TYPE: u32 = 0x80000138;
    pub const WOW_SERVICES_LOGIN_UNABLE_TO_DECODE: u32 = 0x80000139;
    pub const WOW_SERVICES_LOGIN_POST_ERROR: u32 = 0x8000013a;
    pub const WOW_SERVICES_AUTHENTICATOR_PARSE_FAILED: u32 = 0x8000013b;
    pub const WOW_SERVICES_LEGAL_PARSE_FAILED: u32 = 0x8000013c;
    pub const WOW_SERVICES_LOGIN_AUTHENTICATION_PARSE_FAILED: u32 = 0x8000013d;
    pub const WOW_SERIVCES_USER_MUST_ACCEPT_LEGAL: u32 = 0x8000013e;
    pub const WOW_SERVICES_DISCONNECTED: u32 = 0x8000013f;
    pub const WOW_SERVICES_NO_HANDLER_FOR_DISPATCH: u32 = 0x80000140;
    pub const WOW_SERVICES_PRE_DISPATCH_HANDLER_FAILED: u32 = 0x80000141;
    pub const WOW_SERVICES_CRITICAL_STREAMING_ERROR: u32 = 0x80000142;
    pub const WOW_SERVICES_WORLD_LOAD_ERROR: u32 = 0x80000143;
    pub const WOW_SERVICES_LOGIN_FAILED: u32 = 0x80000144;
    pub const WOW_SERVICES_LOGIN_FAILED_ON_CHALLENGE: u32 = 0x80000145;
    pub const WOW_SERVICES_NO_PREPAID_TIME: u32 = 0x80000146;
    pub const WOW_SERVICES_SUBSCRIPTION_EXPIRED: u32 = 0x80000147;
    pub const WOW_SERVICES_CANT_CONNECT: u32 = 0x80000148;
}