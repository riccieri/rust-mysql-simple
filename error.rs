#![macro_escape]

use std::fmt;
use std::io::{IoError};
use super::packet::{ErrPacket};

pub enum MyError {
	MyIoError(IoError),
	MySqlError(ErrPacket),
	MyStrError(~str)
}

impl fmt::Show for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MyIoError(ref io_err) => io_err.fmt(f),
			MySqlError(ref err_packet) => err_packet.fmt(f),
			MyStrError(ref err_str) => write!(f.buf, "{}", err_str)
		}
	}
}

impl<'a> fmt::Show for &'a mut MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

#[macro_export]
macro_rules! try_io(
	($code:expr) => (
		match $code {
			Ok(x) => x,
			Err(e) => return Err(MyIoError(e))
		}
	)
)

pub static ER_HASHCHK: u16 = 1000u16;
pub static ER_NISAMCHK: u16 = 1001u16;
pub static ER_NO: u16 = 1002u16;
pub static ER_YES: u16 = 1003u16;
pub static ER_CANT_CREATE_FILE: u16 = 1004u16;
pub static ER_CANT_CREATE_TABLE: u16 = 1005u16;
pub static ER_CANT_CREATE_DB: u16 = 1006u16;
pub static ER_DB_CREATE_EXISTS: u16 = 1007u16;
pub static ER_DB_DROP_EXISTS: u16 = 1008u16;
pub static ER_DB_DROP_DELETE: u16 = 1009u16;
pub static ER_DB_DROP_RMDIR: u16 = 1010u16;
pub static ER_CANT_DELETE_FILE: u16 = 1011u16;
pub static ER_CANT_FIND_SYSTEM_REC: u16 = 1012u16;
pub static ER_CANT_GET_STAT: u16 = 1013u16;
pub static ER_CANT_GET_WD: u16 = 1014u16;
pub static ER_CANT_LOCK: u16 = 1015u16;
pub static ER_CANT_OPEN_FILE: u16 = 1016u16;
pub static ER_FILE_NOT_FOUND: u16 = 1017u16;
pub static ER_CANT_READ_DIR: u16 = 1018u16;
pub static ER_CANT_SET_WD: u16 = 1019u16;
pub static ER_CHECKREAD: u16 = 1020u16;
pub static ER_DISK_FULL: u16 = 1021u16;
pub static ER_DUP_KEY: u16 = 1022u16;
pub static ER_ERROR_ON_CLOSE: u16 = 1023u16;
pub static ER_ERROR_ON_READ: u16 = 1024u16;
pub static ER_ERROR_ON_RENAME: u16 = 1025u16;
pub static ER_ERROR_ON_WRITE: u16 = 1026u16;
pub static ER_FILE_USED: u16 = 1027u16;
pub static ER_FILSORT_ABORT: u16 = 1028u16;
pub static ER_FORM_NOT_FOUND: u16 = 1029u16;
pub static ER_GET_ERRNO: u16 = 1030u16;
pub static ER_ILLEGAL_HA: u16 = 1031u16;
pub static ER_KEY_NOT_FOUND: u16 = 1032u16;
pub static ER_NOT_FORM_FILE: u16 = 1033u16;
pub static ER_NOT_KEYFILE: u16 = 1034u16;
pub static ER_OLD_KEYFILE: u16 = 1035u16;
pub static ER_OPEN_AS_READONLY: u16 = 1036u16;
pub static ER_OUTOFMEMORY: u16 = 1037u16;
pub static ER_OUT_OF_SORTMEMORY: u16 = 1038u16;
pub static ER_UNEXPECTED_EOF: u16 = 1039u16;
pub static ER_CON_COUNT_ERROR: u16 = 1040u16;
pub static ER_OUT_OF_RESOURCES: u16 = 1041u16;
pub static ER_BAD_HOST_ERROR: u16 = 1042u16;
pub static ER_HANDSHAKE_ERROR: u16 = 1043u16;
pub static ER_DBACCESS_DENIED_ERROR: u16 = 1044u16;
pub static ER_ACCESS_DENIED_ERROR: u16 = 1045u16;
pub static ER_NO_DB_ERROR: u16 = 1046u16;
pub static ER_UNKNOWN_COM_ERROR: u16 = 1047u16;
pub static ER_BAD_NULL_ERROR: u16 = 1048u16;
pub static ER_BAD_DB_ERROR: u16 = 1049u16;
pub static ER_TABLE_EXISTS_ERROR: u16 = 1050u16;
pub static ER_BAD_TABLE_ERROR: u16 = 1051u16;
pub static ER_NON_UNIQ_ERROR: u16 = 1052u16;
pub static ER_SERVER_SHUTDOWN: u16 = 1053u16;
pub static ER_BAD_FIELD_ERROR: u16 = 1054u16;
pub static ER_WRONG_FIELD_WITH_GROUP: u16 = 1055u16;
pub static ER_WRONG_GROUP_FIELD: u16 = 1056u16;
pub static ER_WRONG_SUM_SELECT: u16 = 1057u16;
pub static ER_WRONG_VALUE_COUNT: u16 = 1058u16;
pub static ER_TOO_LONG_IDENT: u16 = 1059u16;
pub static ER_DUP_FIELDNAME: u16 = 1060u16;
pub static ER_DUP_KEYNAME: u16 = 1061u16;
pub static ER_DUP_ENTRY: u16 = 1062u16;
pub static ER_WRONG_FIELD_SPEC: u16 = 1063u16;
pub static ER_PARSE_ERROR: u16 = 1064u16;
pub static ER_EMPTY_QUERY: u16 = 1065u16;
pub static ER_NONUNIQ_TABLE: u16 = 1066u16;
pub static ER_INVALID_DEFAULT: u16 = 1067u16;
pub static ER_MULTIPLE_PRI_KEY: u16 = 1068u16;
pub static ER_TOO_MANY_KEYS: u16 = 1069u16;
pub static ER_TOO_MANY_KEY_PARTS: u16 = 1070u16;
pub static ER_TOO_LONG_KEY: u16 = 1071u16;
pub static ER_KEY_COLUMN_DOES_NOT_EXITS: u16 = 1072u16;
pub static ER_BLOB_USED_AS_KEY: u16 = 1073u16;
pub static ER_TOO_BIG_FIELDLENGTH: u16 = 1074u16;
pub static ER_WRONG_AUTO_KEY: u16 = 1075u16;
pub static ER_READY: u16 = 1076u16;
pub static ER_NORMAL_SHUTDOWN: u16 = 1077u16;
pub static ER_GOT_SIGNAL: u16 = 1078u16;
pub static ER_SHUTDOWN_COMPLETE: u16 = 1079u16;
pub static ER_FORCING_CLOSE: u16 = 1080u16;
pub static ER_IPSOCK_ERROR: u16 = 1081u16;
pub static ER_NO_SUCH_INDEX: u16 = 1082u16;
pub static ER_WRONG_FIELD_TERMINATORS: u16 = 1083u16;
pub static ER_BLOBS_AND_NO_TERMINATED: u16 = 1084u16;
pub static ER_TEXTFILE_NOT_READABLE: u16 = 1085u16;
pub static ER_FILE_EXISTS_ERROR: u16 = 1086u16;
pub static ER_LOAD_INFO: u16 = 1087u16;
pub static ER_ALTER_INFO: u16 = 1088u16;
pub static ER_WRONG_SUB_KEY: u16 = 1089u16;
pub static ER_CANT_REMOVE_ALL_FIELDS: u16 = 1090u16;
pub static ER_CANT_DROP_FIELD_OR_KEY: u16 = 1091u16;
pub static ER_INSERT_INFO: u16 = 1092u16;
pub static ER_UPDATE_TABLE_USED: u16 = 1093u16;
pub static ER_NO_SUCH_THREAD: u16 = 1094u16;
pub static ER_KILL_DENIED_ERROR: u16 = 1095u16;
pub static ER_NO_TABLES_USED: u16 = 1096u16;
pub static ER_TOO_BIG_SET: u16 = 1097u16;
pub static ER_NO_UNIQUE_LOGFILE: u16 = 1098u16;
pub static ER_TABLE_NOT_LOCKED_FOR_WRITE: u16 = 1099u16;
pub static ER_TABLE_NOT_LOCKED: u16 = 1100u16;
pub static ER_BLOB_CANT_HAVE_DEFAULT: u16 = 1101u16;
pub static ER_WRONG_DB_NAME: u16 = 1102u16;
pub static ER_WRONG_TABLE_NAME: u16 = 1103u16;
pub static ER_TOO_BIG_SELECT: u16 = 1104u16;
pub static ER_UNKNOWN_ERROR: u16 = 1105u16;
pub static ER_UNKNOWN_PROCEDURE: u16 = 1106u16;
pub static ER_WRONG_PARAMCOUNT_TO_PROCEDURE: u16 = 1107u16;
pub static ER_WRONG_PARAMETERS_TO_PROCEDURE: u16 = 1108u16;
pub static ER_UNKNOWN_TABLE: u16 = 1109u16;
pub static ER_FIELD_SPECIFIED_TWICE: u16 = 1110u16;
pub static ER_INVALID_GROUP_FUNC_USE: u16 = 1111u16;
pub static ER_UNSUPPORTED_EXTENSION: u16 = 1112u16;
pub static ER_TABLE_MUST_HAVE_COLUMNS: u16 = 1113u16;
pub static ER_RECORD_FILE_FULL: u16 = 1114u16;
pub static ER_UNKNOWN_CHARACTER_SET: u16 = 1115u16;
pub static ER_TOO_MANY_TABLES: u16 = 1116u16;
pub static ER_TOO_MANY_FIELDS: u16 = 1117u16;
pub static ER_TOO_BIG_ROWSIZE: u16 = 1118u16;
pub static ER_STACK_OVERRUN: u16 = 1119u16;
pub static ER_WRONG_OUTER_JOIN: u16 = 1120u16;
pub static ER_NULL_COLUMN_IN_INDEX: u16 = 1121u16;
pub static ER_CANT_FIND_UDF: u16 = 1122u16;
pub static ER_CANT_INITIALIZE_UDF: u16 = 1123u16;
pub static ER_UDF_NO_PATHS: u16 = 1124u16;
pub static ER_UDF_EXISTS: u16 = 1125u16;
pub static ER_CANT_OPEN_LIBRARY: u16 = 1126u16;
pub static ER_CANT_FIND_DL_ENTRY: u16 = 1127u16;
pub static ER_FUNCTION_NOT_DEFINED: u16 = 1128u16;
pub static ER_HOST_IS_BLOCKED: u16 = 1129u16;
pub static ER_HOST_NOT_PRIVILEGED: u16 = 1130u16;
pub static ER_PASSWORD_ANONYMOUS_USER: u16 = 1131u16;
pub static ER_PASSWORD_NOT_ALLOWED: u16 = 1132u16;
pub static ER_PASSWORD_NO_MATCH: u16 = 1133u16;
pub static ER_UPDATE_INFO: u16 = 1134u16;
pub static ER_CANT_CREATE_THREAD: u16 = 1135u16;
pub static ER_WRONG_VALUE_COUNT_ON_ROW: u16 = 1136u16;
pub static ER_CANT_REOPEN_TABLE: u16 = 1137u16;
pub static ER_INVALID_USE_OF_NULL: u16 = 1138u16;
pub static ER_REGEXP_ERROR: u16 = 1139u16;
pub static ER_MIX_OF_GROUP_FUNC_AND_FIELDS: u16 = 1140u16;
pub static ER_NONEXISTING_GRANT: u16 = 1141u16;
pub static ER_TABLEACCESS_DENIED_ERROR: u16 = 1142u16;
pub static ER_COLUMNACCESS_DENIED_ERROR: u16 = 1143u16;
pub static ER_ILLEGAL_GRANT_FOR_TABLE: u16 = 1144u16;
pub static ER_GRANT_WRONG_HOST_OR_USER: u16 = 1145u16;
pub static ER_NO_SUCH_TABLE: u16 = 1146u16;
pub static ER_NONEXISTING_TABLE_GRANT: u16 = 1147u16;
pub static ER_NOT_ALLOWED_COMMAND: u16 = 1148u16;
pub static ER_SYNTAX_ERROR: u16 = 1149u16;
pub static ER_DELAYED_CANT_CHANGE_LOCK: u16 = 1150u16;
pub static ER_TOO_MANY_DELAYED_THREADS: u16 = 1151u16;
pub static ER_ABORTING_CONNECTION: u16 = 1152u16;
pub static ER_NET_PACKET_TOO_LARGE: u16 = 1153u16;
pub static ER_NET_READ_ERROR_FROM_PIPE: u16 = 1154u16;
pub static ER_NET_FCNTL_ERROR: u16 = 1155u16;
pub static ER_NET_PACKETS_OUT_OF_ORDER: u16 = 1156u16;
pub static ER_NET_UNCOMPRESS_ERROR: u16 = 1157u16;
pub static ER_NET_READ_ERROR: u16 = 1158u16;
pub static ER_NET_READ_INTERRUPTED: u16 = 1159u16;
pub static ER_NET_ERROR_ON_WRITE: u16 = 1160u16;
pub static ER_NET_WRITE_INTERRUPTED: u16 = 1161u16;
pub static ER_TOO_LONG_STRING: u16 = 1162u16;
pub static ER_TABLE_CANT_HANDLE_BLOB: u16 = 1163u16;
pub static ER_TABLE_CANT_HANDLE_AUTO_INCREMENT: u16 = 1164u16;
pub static ER_DELAYED_INSERT_TABLE_LOCKED: u16 = 1165u16;
pub static ER_WRONG_COLUMN_NAME: u16 = 1166u16;
pub static ER_WRONG_KEY_COLUMN: u16 = 1167u16;
pub static ER_WRONG_MRG_TABLE: u16 = 1168u16;
pub static ER_DUP_UNIQUE: u16 = 1169u16;
pub static ER_BLOB_KEY_WITHOUT_LENGTH: u16 = 1170u16;
pub static ER_PRIMARY_CANT_HAVE_NULL: u16 = 1171u16;
pub static ER_TOO_MANY_ROWS: u16 = 1172u16;
pub static ER_REQUIRES_PRIMARY_KEY: u16 = 1173u16;
pub static ER_NO_RAID_COMPILED: u16 = 1174u16;
pub static ER_UPDATE_WITHOUT_KEY_IN_SAFE_MODE: u16 = 1175u16;
pub static ER_KEY_DOES_NOT_EXITS: u16 = 1176u16;
pub static ER_CHECK_NO_SUCH_TABLE: u16 = 1177u16;
pub static ER_CHECK_NOT_IMPLEMENTED: u16 = 1178u16;
pub static ER_CANT_DO_THIS_DURING_AN_TRANSACTION: u16 = 1179u16;
pub static ER_ERROR_DURING_COMMIT: u16 = 1180u16;
pub static ER_ERROR_DURING_ROLLBACK: u16 = 1181u16;
pub static ER_ERROR_DURING_FLUSH_LOGS: u16 = 1182u16;
pub static ER_ERROR_DURING_CHECKPOINT: u16 = 1183u16;
pub static ER_NEW_ABORTING_CONNECTION: u16 = 1184u16;
pub static ER_DUMP_NOT_IMPLEMENTED: u16 = 1185u16;
pub static ER_FLUSH_MASTER_BINLOG_CLOSED: u16 = 1186u16;
pub static ER_INDEX_REBUILD: u16 = 1187u16;
pub static ER_MASTER: u16 = 1188u16;
pub static ER_MASTER_NET_READ: u16 = 1189u16;
pub static ER_MASTER_NET_WRITE: u16 = 1190u16;
pub static ER_FT_MATCHING_KEY_NOT_FOUND: u16 = 1191u16;
pub static ER_LOCK_OR_ACTIVE_TRANSACTION: u16 = 1192u16;
pub static ER_UNKNOWN_SYSTEM_VARIABLE: u16 = 1193u16;
pub static ER_CRASHED_ON_USAGE: u16 = 1194u16;
pub static ER_CRASHED_ON_REPAIR: u16 = 1195u16;
pub static ER_WARNING_NOT_COMPLETE_ROLLBACK: u16 = 1196u16;
pub static ER_TRANS_CACHE_FULL: u16 = 1197u16;
pub static ER_SLAVE_MUST_STOP: u16 = 1198u16;
pub static ER_SLAVE_NOT_RUNNING: u16 = 1199u16;
pub static ER_BAD_SLAVE: u16 = 1200u16;
pub static ER_MASTER_INFO: u16 = 1201u16;
pub static ER_SLAVE_THREAD: u16 = 1202u16;
pub static ER_TOO_MANY_USER_CONNECTIONS: u16 = 1203u16;
pub static ER_SET_CONSTANTS_ONLY: u16 = 1204u16;
pub static ER_LOCK_WAIT_TIMEOUT: u16 = 1205u16;
pub static ER_LOCK_TABLE_FULL: u16 = 1206u16;
pub static ER_READ_ONLY_TRANSACTION: u16 = 1207u16;
pub static ER_DROP_DB_WITH_READ_LOCK: u16 = 1208u16;
pub static ER_CREATE_DB_WITH_READ_LOCK: u16 = 1209u16;
pub static ER_WRONG_ARGUMENTS: u16 = 1210u16;
pub static ER_NO_PERMISSION_TO_CREATE_USER: u16 = 1211u16;
pub static ER_UNION_TABLES_IN_DIFFERENT_DIR: u16 = 1212u16;
pub static ER_LOCK_DEADLOCK: u16 = 1213u16;
pub static ER_TABLE_CANT_HANDLE_FT: u16 = 1214u16;
pub static ER_CANNOT_ADD_FOREIGN: u16 = 1215u16;
pub static ER_NO_REFERENCED_ROW: u16 = 1216u16;
pub static ER_ROW_IS_REFERENCED: u16 = 1217u16;
pub static ER_CONNECT_TO_MASTER: u16 = 1218u16;
pub static ER_QUERY_ON_MASTER: u16 = 1219u16;
pub static ER_ERROR_WHEN_EXECUTING_COMMAND: u16 = 1220u16;
pub static ER_WRONG_USAGE: u16 = 1221u16;
pub static ER_WRONG_NUMBER_OF_COLUMNS_IN_SELECT: u16 = 1222u16;
pub static ER_CANT_UPDATE_WITH_READLOCK: u16 = 1223u16;
pub static ER_MIXING_NOT_ALLOWED: u16 = 1224u16;
pub static ER_DUP_ARGUMENT: u16 = 1225u16;
pub static ER_USER_LIMIT_REACHED: u16 = 1226u16;
pub static ER_SPECIFIC_ACCESS_DENIED_ERROR: u16 = 1227u16;
pub static ER_LOCAL_VARIABLE: u16 = 1228u16;
pub static ER_GLOBAL_VARIABLE: u16 = 1229u16;
pub static ER_NO_DEFAULT: u16 = 1230u16;
pub static ER_WRONG_VALUE_FOR_VAR: u16 = 1231u16;
pub static ER_WRONG_TYPE_FOR_VAR: u16 = 1232u16;
pub static ER_VAR_CANT_BE_READ: u16 = 1233u16;
pub static ER_CANT_USE_OPTION_HERE: u16 = 1234u16;
pub static ER_NOT_SUPPORTED_YET: u16 = 1235u16;
pub static ER_MASTER_FATAL_ERROR_READING_BINLOG: u16 = 1236u16;
pub static ER_SLAVE_IGNORED_TABLE: u16 = 1237u16;
pub static ER_INCORRECT_GLOBAL_LOCAL_VAR: u16 = 1238u16;
pub static ER_WRONG_FK_DEF: u16 = 1239u16;
pub static ER_KEY_REF_DO_NOT_MATCH_TABLE_REF: u16 = 1240u16;
pub static ER_OPERAND_COLUMNS: u16 = 1241u16;
pub static ER_SUBQUERY_NO_1_ROW: u16 = 1242u16;
pub static ER_UNKNOWN_STMT_HANDLER: u16 = 1243u16;
pub static ER_CORRUPT_HELP_DB: u16 = 1244u16;
pub static ER_CYCLIC_REFERENCE: u16 = 1245u16;
pub static ER_AUTO_CONVERT: u16 = 1246u16;
pub static ER_ILLEGAL_REFERENCE: u16 = 1247u16;
pub static ER_DERIVED_MUST_HAVE_ALIAS: u16 = 1248u16;
pub static ER_SELECT_REDUCED: u16 = 1249u16;
pub static ER_TABLENAME_NOT_ALLOWED_HERE: u16 = 1250u16;
pub static ER_NOT_SUPPORTED_AUTH_MODE: u16 = 1251u16;
pub static ER_SPATIAL_CANT_HAVE_NULL: u16 = 1252u16;
pub static ER_COLLATION_CHARSET_MISMATCH: u16 = 1253u16;
pub static ER_SLAVE_WAS_RUNNING: u16 = 1254u16;
pub static ER_SLAVE_WAS_NOT_RUNNING: u16 = 1255u16;
pub static ER_TOO_BIG_FOR_UNCOMPRESS: u16 = 1256u16;
pub static ER_ZLIB_Z_MEM_ERROR: u16 = 1257u16;
pub static ER_ZLIB_Z_BUF_ERROR: u16 = 1258u16;
pub static ER_ZLIB_Z_DATA_ERROR: u16 = 1259u16;
pub static ER_CUT_VALUE_GROUP_CONCAT: u16 = 1260u16;
pub static ER_WARN_TOO_FEW_RECORDS: u16 = 1261u16;
pub static ER_WARN_TOO_MANY_RECORDS: u16 = 1262u16;
pub static ER_WARN_NULL_TO_NOTNULL: u16 = 1263u16;
pub static ER_WARN_DATA_OUT_OF_RANGE: u16 = 1264u16;
pub static WARN_DATA_TRUNCATED: u16 = 1265u16;
pub static ER_WARN_USING_OTHER_HANDLER: u16 = 1266u16;
pub static ER_CANT_AGGREGATE_2COLLATIONS: u16 = 1267u16;
pub static ER_DROP_USER: u16 = 1268u16;
pub static ER_REVOKE_GRANTS: u16 = 1269u16;
pub static ER_CANT_AGGREGATE_3COLLATIONS: u16 = 1270u16;
pub static ER_CANT_AGGREGATE_NCOLLATIONS: u16 = 1271u16;
pub static ER_VARIABLE_IS_NOT_STRUCT: u16 = 1272u16;
pub static ER_UNKNOWN_COLLATION: u16 = 1273u16;
pub static ER_SLAVE_IGNORED_SSL_PARAMS: u16 = 1274u16;
pub static ER_SERVER_IS_IN_SECURE_AUTH_MODE: u16 = 1275u16;
pub static ER_WARN_FIELD_RESOLVED: u16 = 1276u16;
pub static ER_BAD_SLAVE_UNTIL_COND: u16 = 1277u16;
pub static ER_MISSING_SKIP_SLAVE: u16 = 1278u16;
pub static ER_UNTIL_COND_IGNORED: u16 = 1279u16;
pub static ER_WRONG_NAME_FOR_INDEX: u16 = 1280u16;
pub static ER_WRONG_NAME_FOR_CATALOG: u16 = 1281u16;
pub static ER_WARN_QC_RESIZE: u16 = 1282u16;
pub static ER_BAD_FT_COLUMN: u16 = 1283u16;
pub static ER_UNKNOWN_KEY_CACHE: u16 = 1284u16;
pub static ER_WARN_HOSTNAME_WONT_WORK: u16 = 1285u16;
pub static ER_UNKNOWN_STORAGE_ENGINE: u16 = 1286u16;
pub static ER_WARN_DEPRECATED_SYNTAX: u16 = 1287u16;
pub static ER_NON_UPDATABLE_TABLE: u16 = 1288u16;
pub static ER_FEATURE_DISABLED: u16 = 1289u16;
pub static ER_OPTION_PREVENTS_STATEMENT: u16 = 1290u16;
pub static ER_DUPLICATED_VALUE_IN_TYPE: u16 = 1291u16;
pub static ER_TRUNCATED_WRONG_VALUE: u16 = 1292u16;
pub static ER_TOO_MUCH_AUTO_TIMESTAMP_COLS: u16 = 1293u16;
pub static ER_INVALID_ON_UPDATE: u16 = 1294u16;
pub static ER_UNSUPPORTED_PS: u16 = 1295u16;
pub static ER_GET_ERRMSG: u16 = 1296u16;
pub static ER_GET_TEMPORARY_ERRMSG: u16 = 1297u16;
pub static ER_UNKNOWN_TIME_ZONE: u16 = 1298u16;
pub static ER_WARN_INVALID_TIMESTAMP: u16 = 1299u16;
pub static ER_INVALID_CHARACTER_STRING: u16 = 1300u16;
pub static ER_WARN_ALLOWED_PACKET_OVERFLOWED: u16 = 1301u16;
pub static ER_CONFLICTING_DECLARATIONS: u16 = 1302u16;
pub static ER_SP_NO_RECURSIVE_CREATE: u16 = 1303u16;
pub static ER_SP_ALREADY_EXISTS: u16 = 1304u16;
pub static ER_SP_DOES_NOT_EXIST: u16 = 1305u16;
pub static ER_SP_DROP_FAILED: u16 = 1306u16;
pub static ER_SP_STORE_FAILED: u16 = 1307u16;
pub static ER_SP_LILABEL_MISMATCH: u16 = 1308u16;
pub static ER_SP_LABEL_REDEFINE: u16 = 1309u16;
pub static ER_SP_LABEL_MISMATCH: u16 = 1310u16;
pub static ER_SP_UNINIT_VAR: u16 = 1311u16;
pub static ER_SP_BADSELECT: u16 = 1312u16;
pub static ER_SP_BADRETURN: u16 = 1313u16;
pub static ER_SP_BADSTATEMENT: u16 = 1314u16;
pub static ER_UPDATE_LOG_DEPRECATED_IGNORED: u16 = 1315u16;
pub static ER_UPDATE_LOG_DEPRECATED_TRANSLATED: u16 = 1316u16;
pub static ER_QUERY_INTERRUPTED: u16 = 1317u16;
pub static ER_SP_WRONG_NO_OF_ARGS: u16 = 1318u16;
pub static ER_SP_COND_MISMATCH: u16 = 1319u16;
pub static ER_SP_NORETURN: u16 = 1320u16;
pub static ER_SP_NORETURNEND: u16 = 1321u16;
pub static ER_SP_BAD_CURSOR_QUERY: u16 = 1322u16;
pub static ER_SP_BAD_CURSOR_SELECT: u16 = 1323u16;
pub static ER_SP_CURSOR_MISMATCH: u16 = 1324u16;
pub static ER_SP_CURSOR_ALREADY_OPEN: u16 = 1325u16;
pub static ER_SP_CURSOR_NOT_OPEN: u16 = 1326u16;
pub static ER_SP_UNDECLARED_VAR: u16 = 1327u16;
pub static ER_SP_WRONG_NO_OF_FETCH_ARGS: u16 = 1328u16;
pub static ER_SP_FETCH_NO_DATA: u16 = 1329u16;
pub static ER_SP_DUP_PARAM: u16 = 1330u16;
pub static ER_SP_DUP_VAR: u16 = 1331u16;
pub static ER_SP_DUP_COND: u16 = 1332u16;
pub static ER_SP_DUP_CURS: u16 = 1333u16;
pub static ER_SP_CANT_ALTER: u16 = 1334u16;
pub static ER_SP_SUBSELECT_NYI: u16 = 1335u16;
pub static ER_STMT_NOT_ALLOWED_IN_SF_OR_TRG: u16 = 1336u16;
pub static ER_SP_VARCOND_AFTER_CURSHNDLR: u16 = 1337u16;
pub static ER_SP_CURSOR_AFTER_HANDLER: u16 = 1338u16;
pub static ER_SP_CASE_NOT_FOUND: u16 = 1339u16;
pub static ER_FPARSER_TOO_BIG_FILE: u16 = 1340u16;
pub static ER_FPARSER_BAD_HEADER: u16 = 1341u16;
pub static ER_FPARSER_EOF_IN_COMMENT: u16 = 1342u16;
pub static ER_FPARSER_ERROR_IN_PARAMETER: u16 = 1343u16;
pub static ER_FPARSER_EOF_IN_UNKNOWN_PARAMETER: u16 = 1344u16;
pub static ER_VIEW_NO_EXPLAIN: u16 = 1345u16;
pub static ER_FRM_UNKNOWN_TYPE: u16 = 1346u16;
pub static ER_WRONG_OBJECT: u16 = 1347u16;
pub static ER_NONUPDATEABLE_COLUMN: u16 = 1348u16;
pub static ER_VIEW_SELECT_DERIVED: u16 = 1349u16;
pub static ER_VIEW_SELECT_CLAUSE: u16 = 1350u16;
pub static ER_VIEW_SELECT_VARIABLE: u16 = 1351u16;
pub static ER_VIEW_SELECT_TMPTABLE: u16 = 1352u16;
pub static ER_VIEW_WRONG_LIST: u16 = 1353u16;
pub static ER_WARN_VIEW_MERGE: u16 = 1354u16;
pub static ER_WARN_VIEW_WITHOUT_KEY: u16 = 1355u16;
pub static ER_VIEW_INVALID: u16 = 1356u16;
pub static ER_SP_NO_DROP_SP: u16 = 1357u16;
pub static ER_SP_GOTO_IN_HNDLR: u16 = 1358u16;
pub static ER_TRG_ALREADY_EXISTS: u16 = 1359u16;
pub static ER_TRG_DOES_NOT_EXIST: u16 = 1360u16;
pub static ER_TRG_ON_VIEW_OR_TEMP_TABLE: u16 = 1361u16;
pub static ER_TRG_CANT_CHANGE_ROW: u16 = 1362u16;
pub static ER_TRG_NO_SUCH_ROW_IN_TRG: u16 = 1363u16;
pub static ER_NO_DEFAULT_FOR_FIELD: u16 = 1364u16;
pub static ER_DIVISION_BY_ZERO: u16 = 1365u16;
pub static ER_TRUNCATED_WRONG_VALUE_FOR_FIELD: u16 = 1366u16;
pub static ER_ILLEGAL_VALUE_FOR_TYPE: u16 = 1367u16;
pub static ER_VIEW_NONUPD_CHECK: u16 = 1368u16;
pub static ER_VIEW_CHECK_FAILED: u16 = 1369u16;
pub static ER_PROCACCESS_DENIED_ERROR: u16 = 1370u16;
pub static ER_RELAY_LOG_FAIL: u16 = 1371u16;
pub static ER_PASSWD_LENGTH: u16 = 1372u16;
pub static ER_UNKNOWN_TARGET_BINLOG: u16 = 1373u16;
pub static ER_IO_ERR_LOG_INDEX_READ: u16 = 1374u16;
pub static ER_BINLOG_PURGE_PROHIBITED: u16 = 1375u16;
pub static ER_FSEEK_FAIL: u16 = 1376u16;
pub static ER_BINLOG_PURGE_FATAL_ERR: u16 = 1377u16;
pub static ER_LOG_IN_USE: u16 = 1378u16;
pub static ER_LOG_PURGE_UNKNOWN_ERR: u16 = 1379u16;
pub static ER_RELAY_LOG_INIT: u16 = 1380u16;
pub static ER_NO_BINARY_LOGGING: u16 = 1381u16;
pub static ER_RESERVED_SYNTAX: u16 = 1382u16;
pub static ER_WSAS_FAILED: u16 = 1383u16;
pub static ER_DIFF_GROUPS_PROC: u16 = 1384u16;
pub static ER_NO_GROUP_FOR_PROC: u16 = 1385u16;
pub static ER_ORDER_WITH_PROC: u16 = 1386u16;
pub static ER_LOGGING_PROHIBIT_CHANGING_OF: u16 = 1387u16;
pub static ER_NO_FILE_MAPPING: u16 = 1388u16;
pub static ER_WRONG_MAGIC: u16 = 1389u16;
pub static ER_PS_MANY_PARAM: u16 = 1390u16;
pub static ER_KEY_PART_0: u16 = 1391u16;
pub static ER_VIEW_CHECKSUM: u16 = 1392u16;
pub static ER_VIEW_MULTIUPDATE: u16 = 1393u16;
pub static ER_VIEW_NO_INSERT_FIELD_LIST: u16 = 1394u16;
pub static ER_VIEW_DELETE_MERGE_VIEW: u16 = 1395u16;
pub static ER_CANNOT_USER: u16 = 1396u16;
pub static ER_XAER_NOTA: u16 = 1397u16;
pub static ER_XAER_INVAL: u16 = 1398u16;
pub static ER_XAER_RMFAIL: u16 = 1399u16;
pub static ER_XAER_OUTSIDE: u16 = 1400u16;
pub static ER_XAER_RMERR: u16 = 1401u16;
pub static ER_XA_RBROLLBACK: u16 = 1402u16;
pub static ER_NONEXISTING_PROC_GRANT: u16 = 1403u16;
pub static ER_PROC_AUTO_GRANT_FAIL: u16 = 1404u16;
pub static ER_PROC_AUTO_REVOKE_FAIL: u16 = 1405u16;
pub static ER_DATA_TOO_LONG: u16 = 1406u16;
pub static ER_SP_BAD_SQLSTATE: u16 = 1407u16;
pub static ER_STARTUP: u16 = 1408u16;
pub static ER_LOAD_FROM_FIXED_SIZE_ROWS_TO_VAR: u16 = 1409u16;
pub static ER_CANT_CREATE_USER_WITH_GRANT: u16 = 1410u16;
pub static ER_WRONG_VALUE_FOR_TYPE: u16 = 1411u16;
pub static ER_TABLE_DEF_CHANGED: u16 = 1412u16;
pub static ER_SP_DUP_HANDLER: u16 = 1413u16;
pub static ER_SP_NOT_VAR_ARG: u16 = 1414u16;
pub static ER_SP_NO_RETSET: u16 = 1415u16;
pub static ER_CANT_CREATE_GEOMETRY_OBJECT: u16 = 1416u16;
pub static ER_FAILED_ROUTINE_BREAK_BINLOG: u16 = 1417u16;
pub static ER_BINLOG_UNSAFE_ROUTINE: u16 = 1418u16;
pub static ER_BINLOG_CREATE_ROUTINE_NEED_SUPER: u16 = 1419u16;
pub static ER_EXEC_STMT_WITH_OPEN_CURSOR: u16 = 1420u16;
pub static ER_STMT_HAS_NO_OPEN_CURSOR: u16 = 1421u16;
pub static ER_COMMIT_NOT_ALLOWED_IN_SF_OR_TRG: u16 = 1422u16;
pub static ER_NO_DEFAULT_FOR_VIEW_FIELD: u16 = 1423u16;
pub static ER_SP_NO_RECURSION: u16 = 1424u16;
pub static ER_TOO_BIG_SCALE: u16 = 1425u16;
pub static ER_TOO_BIG_PRECISION: u16 = 1426u16;
pub static ER_M_BIGGER_THAN_D: u16 = 1427u16;
pub static ER_WRONG_LOCK_OF_SYSTEM_TABLE: u16 = 1428u16;
pub static ER_CONNECT_TO_FOREIGN_DATA_SOURCE: u16 = 1429u16;
pub static ER_QUERY_ON_FOREIGN_DATA_SOURCE: u16 = 1430u16;
pub static ER_FOREIGN_DATA_SOURCE_DOESNT_EXIST: u16 = 1431u16;
pub static ER_FOREIGN_DATA_STRING_INVALID_CANT_CREATE: u16 = 1432u16;
pub static ER_FOREIGN_DATA_STRING_INVALID: u16 = 1433u16;
pub static ER_CANT_CREATE_FEDERATED_TABLE: u16 = 1434u16;
pub static ER_TRG_IN_WRONG_SCHEMA: u16 = 1435u16;
pub static ER_STACK_OVERRUN_NEED_MORE: u16 = 1436u16;
pub static ER_TOO_LONG_BODY: u16 = 1437u16;
pub static ER_WARN_CANT_DROP_DEFAULT_KEYCACHE: u16 = 1438u16;
pub static ER_TOO_BIG_DISPLAYWIDTH: u16 = 1439u16;
pub static ER_XAER_DUPID: u16 = 1440u16;
pub static ER_DATETIME_FUNCTION_OVERFLOW: u16 = 1441u16;
pub static ER_CANT_UPDATE_USED_TABLE_IN_SF_OR_TRG: u16 = 1442u16;
pub static ER_VIEW_PREVENT_UPDATE: u16 = 1443u16;
pub static ER_PS_NO_RECURSION: u16 = 1444u16;
pub static ER_SP_CANT_SET_AUTOCOMMIT: u16 = 1445u16;
pub static ER_MALFORMED_DEFINER: u16 = 1446u16;
pub static ER_VIEW_FRM_NO_USER: u16 = 1447u16;
pub static ER_VIEW_OTHER_USER: u16 = 1448u16;
pub static ER_NO_SUCH_USER: u16 = 1449u16;
pub static ER_FORBID_SCHEMA_CHANGE: u16 = 1450u16;
pub static ER_ROW_IS_REFERENCED_2: u16 = 1451u16;
pub static ER_NO_REFERENCED_ROW_2: u16 = 1452u16;
pub static ER_SP_BAD_VAR_SHADOW: u16 = 1453u16;
pub static ER_TRG_NO_DEFINER: u16 = 1454u16;
pub static ER_OLD_FILE_FORMAT: u16 = 1455u16;
pub static ER_SP_RECURSION_LIMIT: u16 = 1456u16;
pub static ER_SP_PROC_TABLE_CORRUPT: u16 = 1457u16;
pub static ER_SP_WRONG_NAME: u16 = 1458u16;
pub static ER_TABLE_NEEDS_UPGRADE: u16 = 1459u16;
pub static ER_SP_NO_AGGREGATE: u16 = 1460u16;
pub static ER_MAX_PREPARED_STMT_COUNT_REACHED: u16 = 1461u16;
pub static ER_VIEW_RECURSIVE: u16 = 1462u16;
pub static ER_NON_GROUPING_FIELD_USED: u16 = 1463u16;
pub static ER_TABLE_CANT_HANDLE_SPKEYS: u16 = 1464u16;
pub static ER_NO_TRIGGERS_ON_SYSTEM_SCHEMA: u16 = 1465u16;
pub static ER_REMOVED_SPACES: u16 = 1466u16;
pub static ER_AUTOINC_READ_FAILED: u16 = 1467u16;
pub static ER_USERNAME: u16 = 1468u16;
pub static ER_HOSTNAME: u16 = 1469u16;
pub static ER_WRONG_STRING_LENGTH: u16 = 1470u16;
pub static ER_NON_INSERTABLE_TABLE: u16 = 1471u16;
pub static ER_ADMIN_WRONG_MRG_TABLE: u16 = 1472u16;
pub static ER_TOO_HIGH_LEVEL_OF_NESTING_FOR_SELECT: u16 = 1473u16;
pub static ER_NAME_BECOMES_EMPTY: u16 = 1474u16;
pub static ER_AMBIGUOUS_FIELD_TERM: u16 = 1475u16;
pub static ER_FOREIGN_SERVER_EXISTS: u16 = 1476u16;
pub static ER_FOREIGN_SERVER_DOESNT_EXIST: u16 = 1477u16;
pub static ER_ILLEGAL_HA_CREATE_OPTION: u16 = 1478u16;
pub static ER_PARTITION_REQUIRES_VALUES_ERROR: u16 = 1479u16;
pub static ER_PARTITION_WRONG_VALUES_ERROR: u16 = 1480u16;
pub static ER_PARTITION_MAXVALUE_ERROR: u16 = 1481u16;
pub static ER_PARTITION_SUBPARTITION_ERROR: u16 = 1482u16;
pub static ER_PARTITION_SUBPART_MIX_ERROR: u16 = 1483u16;
pub static ER_PARTITION_WRONG_NO_PART_ERROR: u16 = 1484u16;
pub static ER_PARTITION_WRONG_NO_SUBPART_ERROR: u16 = 1485u16;
pub static ER_CONST_EXPR_IN_PARTITION_FUNC_ERROR: u16 = 1486u16;
pub static ER_WRONG_EXPR_IN_PARTITION_FUNC_ERROR: u16 = 1486u16;
pub static ER_NO_CONST_EXPR_IN_RANGE_OR_LIST_ERROR: u16 = 1487u16;
pub static ER_FIELD_NOT_FOUND_PART_ERROR: u16 = 1488u16;
pub static ER_LIST_OF_FIELDS_ONLY_IN_HASH_ERROR: u16 = 1489u16;
pub static ER_INCONSISTENT_PARTITION_INFO_ERROR: u16 = 1490u16;
pub static ER_PARTITION_FUNC_NOT_ALLOWED_ERROR: u16 = 1491u16;
pub static ER_PARTITIONS_MUST_BE_DEFINED_ERROR: u16 = 1492u16;
pub static ER_RANGE_NOT_INCREASING_ERROR: u16 = 1493u16;
pub static ER_INCONSISTENT_TYPE_OF_FUNCTIONS_ERROR: u16 = 1494u16;
pub static ER_MULTIPLE_DEF_CONST_IN_LIST_PART_ERROR: u16 = 1495u16;
pub static ER_PARTITION_ENTRY_ERROR: u16 = 1496u16;
pub static ER_MIX_HANDLER_ERROR: u16 = 1497u16;
pub static ER_PARTITION_NOT_DEFINED_ERROR: u16 = 1498u16;
pub static ER_TOO_MANY_PARTITIONS_ERROR: u16 = 1499u16;
pub static ER_SUBPARTITION_ERROR: u16 = 1500u16;
pub static ER_CANT_CREATE_HANDLER_FILE: u16 = 1501u16;
pub static ER_BLOB_FIELD_IN_PART_FUNC_ERROR: u16 = 1502u16;
pub static ER_UNIQUE_KEY_NEED_ALL_FIELDS_IN_PF: u16 = 1503u16;
pub static ER_NO_PARTS_ERROR: u16 = 1504u16;
pub static ER_PARTITION_MGMT_ON_NONPARTITIONED: u16 = 1505u16;
pub static ER_FOREIGN_KEY_ON_PARTITIONED: u16 = 1506u16;
pub static ER_DROP_PARTITION_NON_EXISTENT: u16 = 1507u16;
pub static ER_DROP_LAST_PARTITION: u16 = 1508u16;
pub static ER_COALESCE_ONLY_ON_HASH_PARTITION: u16 = 1509u16;
pub static ER_REORG_HASH_ONLY_ON_SAME_NO: u16 = 1510u16;
pub static ER_REORG_NO_PARAM_ERROR: u16 = 1511u16;
pub static ER_ONLY_ON_RANGE_LIST_PARTITION: u16 = 1512u16;
pub static ER_ADD_PARTITION_SUBPART_ERROR: u16 = 1513u16;
pub static ER_ADD_PARTITION_NO_NEW_PARTITION: u16 = 1514u16;
pub static ER_COALESCE_PARTITION_NO_PARTITION: u16 = 1515u16;
pub static ER_REORG_PARTITION_NOT_EXIST: u16 = 1516u16;
pub static ER_SAME_NAME_PARTITION: u16 = 1517u16;
pub static ER_NO_BINLOG_ERROR: u16 = 1518u16;
pub static ER_CONSECUTIVE_REORG_PARTITIONS: u16 = 1519u16;
pub static ER_REORG_OUTSIDE_RANGE: u16 = 1520u16;
pub static ER_PARTITION_FUNCTION_FAILURE: u16 = 1521u16;
pub static ER_PART_STATE_ERROR: u16 = 1522u16;
pub static ER_LIMITED_PART_RANGE: u16 = 1523u16;
pub static ER_PLUGIN_IS_NOT_LOADED: u16 = 1524u16;
pub static ER_WRONG_VALUE: u16 = 1525u16;
pub static ER_NO_PARTITION_FOR_GIVEN_VALUE: u16 = 1526u16;
pub static ER_FILEGROUP_OPTION_ONLY_ONCE: u16 = 1527u16;
pub static ER_CREATE_FILEGROUP_FAILED: u16 = 1528u16;
pub static ER_DROP_FILEGROUP_FAILED: u16 = 1529u16;
pub static ER_TABLESPACE_AUTO_EXTEND_ERROR: u16 = 1530u16;
pub static ER_WRONG_SIZE_NUMBER: u16 = 1531u16;
pub static ER_SIZE_OVERFLOW_ERROR: u16 = 1532u16;
pub static ER_ALTER_FILEGROUP_FAILED: u16 = 1533u16;
pub static ER_BINLOG_ROW_LOGGING_FAILED: u16 = 1534u16;
pub static ER_BINLOG_ROW_WRONG_TABLE_DEF: u16 = 1535u16;
pub static ER_BINLOG_ROW_RBR_TO_SBR: u16 = 1536u16;
pub static ER_EVENT_ALREADY_EXISTS: u16 = 1537u16;
pub static ER_EVENT_STORE_FAILED: u16 = 1538u16;
pub static ER_EVENT_DOES_NOT_EXIST: u16 = 1539u16;
pub static ER_EVENT_CANT_ALTER: u16 = 1540u16;
pub static ER_EVENT_DROP_FAILED: u16 = 1541u16;
pub static ER_EVENT_INTERVAL_NOT_POSITIVE_OR_TOO_BIG: u16 = 1542u16;
pub static ER_EVENT_ENDS_BEFORE_STARTS: u16 = 1543u16;
pub static ER_EVENT_EXEC_TIME_IN_THE_PAST: u16 = 1544u16;
pub static ER_EVENT_OPEN_TABLE_FAILED: u16 = 1545u16;
pub static ER_EVENT_NEITHER_M_EXPR_NOR_M_AT: u16 = 1546u16;
pub static ER_COL_COUNT_DOESNT_MATCH_CORRUPTED: u16 = 1547u16;
pub static ER_CANNOT_LOAD_FROM_TABLE: u16 = 1548u16;
pub static ER_EVENT_CANNOT_DELETE: u16 = 1549u16;
pub static ER_EVENT_COMPILE_ERROR: u16 = 1550u16;
pub static ER_EVENT_SAME_NAME: u16 = 1551u16;
pub static ER_EVENT_DATA_TOO_LONG: u16 = 1552u16;
pub static ER_DROP_INDEX_FK: u16 = 1553u16;
pub static ER_WARN_DEPRECATED_SYNTAX_WITH_VER: u16 = 1554u16;
pub static ER_CANT_WRITE_LOCK_LOG_TABLE: u16 = 1555u16;
pub static ER_CANT_LOCK_LOG_TABLE: u16 = 1556u16;
pub static ER_FOREIGN_DUPLICATE_KEY: u16 = 1557u16;
pub static ER_COL_COUNT_DOESNT_MATCH_PLEASE_UPDATE: u16 = 1558u16;
pub static ER_TEMP_TABLE_PREVENTS_SWITCH_OUT_OF_RBR: u16 = 1559u16;
pub static ER_STORED_FUNCTION_PREVENTS_SWITCH_BINLOG_FORMAT: u16 = 1560u16;
pub static ER_NDB_CANT_SWITCH_BINLOG_FORMAT: u16 = 1561u16;
pub static ER_PARTITION_NO_TEMPORARY: u16 = 1562u16;
pub static ER_PARTITION_CONST_DOMAIN_ERROR: u16 = 1563u16;
pub static ER_PARTITION_FUNCTION_IS_NOT_ALLOWED: u16 = 1564u16;
pub static ER_DDL_LOG_ERROR: u16 = 1565u16;
pub static ER_NULL_IN_VALUES_LESS_THAN: u16 = 1566u16;
pub static ER_WRONG_PARTITION_NAME: u16 = 1567u16;
pub static ER_CANT_CHANGE_TX_ISOLATION: u16 = 1568u16;
pub static ER_DUP_ENTRY_AUTOINCREMENT_CASE: u16 = 1569u16;
pub static ER_EVENT_MODIFY_QUEUE_ERROR: u16 = 1570u16;
pub static ER_EVENT_SET_VAR_ERROR: u16 = 1571u16;
pub static ER_PARTITION_MERGE_ERROR: u16 = 1572u16;
pub static ER_CANT_ACTIVATE_LOG: u16 = 1573u16;
pub static ER_RBR_NOT_AVAILABLE: u16 = 1574u16;
pub static ER_BASE64_DECODE_ERROR: u16 = 1575u16;
pub static ER_EVENT_RECURSION_FORBIDDEN: u16 = 1576u16;
pub static ER_EVENTS_DB_ERROR: u16 = 1577u16;
pub static ER_ONLY_INTEGERS_ALLOWED: u16 = 1578u16;
pub static ER_UNSUPORTED_LOG_ENGINE: u16 = 1579u16;
pub static ER_BAD_LOG_STATEMENT: u16 = 1580u16;
pub static ER_CANT_RENAME_LOG_TABLE: u16 = 1581u16;
pub static ER_WRONG_PARAMCOUNT_TO_NATIVE_FCT: u16 = 1582u16;
pub static ER_WRONG_PARAMETERS_TO_NATIVE_FCT: u16 = 1583u16;
pub static ER_WRONG_PARAMETERS_TO_STORED_FCT: u16 = 1584u16;
pub static ER_NATIVE_FCT_NAME_COLLISION: u16 = 1585u16;
pub static ER_DUP_ENTRY_WITH_KEY_NAME: u16 = 1586u16;
pub static ER_BINLOG_PURGE_EMFILE: u16 = 1587u16;
pub static ER_EVENT_CANNOT_CREATE_IN_THE_PAST: u16 = 1588u16;
pub static ER_EVENT_CANNOT_ALTER_IN_THE_PAST: u16 = 1589u16;
pub static ER_SLAVE_INCIDENT: u16 = 1590u16;
pub static ER_NO_PARTITION_FOR_GIVEN_VALUE_SILENT: u16 = 1591u16;
pub static ER_BINLOG_UNSAFE_STATEMENT: u16 = 1592u16;
pub static ER_SLAVE_FATAL_ERROR: u16 = 1593u16;
pub static ER_SLAVE_RELAY_LOG_READ_FAILURE: u16 = 1594u16;
pub static ER_SLAVE_RELAY_LOG_WRITE_FAILURE: u16 = 1595u16;
pub static ER_SLAVE_CREATE_EVENT_FAILURE: u16 = 1596u16;
pub static ER_SLAVE_MASTER_COM_FAILURE: u16 = 1597u16;
pub static ER_BINLOG_LOGGING_IMPOSSIBLE: u16 = 1598u16;
pub static ER_VIEW_NO_CREATION_CTX: u16 = 1599u16;
pub static ER_VIEW_INVALID_CREATION_CTX: u16 = 1600u16;
pub static ER_SR_INVALID_CREATION_CTX: u16 = 1601u16;
pub static ER_TRG_CORRUPTED_FILE: u16 = 1602u16;
pub static ER_TRG_NO_CREATION_CTX: u16 = 1603u16;
pub static ER_TRG_INVALID_CREATION_CTX: u16 = 1604u16;
pub static ER_EVENT_INVALID_CREATION_CTX: u16 = 1605u16;
pub static ER_TRG_CANT_OPEN_TABLE: u16 = 1606u16;
pub static ER_CANT_CREATE_SROUTINE: u16 = 1607u16;
pub static ER_SLAVE_AMBIGOUS_EXEC_MODE: u16 = 1608u16;
pub static ER_NEVER_USED: u16 = 1608u16;
pub static ER_NO_FORMAT_DESCRIPTION_EVENT_BEFORE_BINLOG_STATEMENT: u16 = 1609u16;
pub static ER_SLAVE_CORRUPT_EVENT: u16 = 1610u16;
pub static ER_LOAD_DATA_INVALID_COLUMN: u16 = 1611u16;
pub static ER_LOG_PURGE_NO_FILE: u16 = 1612u16;
pub static ER_XA_RBTIMEOUT: u16 = 1613u16;
pub static ER_XA_RBDEADLOCK: u16 = 1614u16;
pub static ER_NEED_REPREPARE: u16 = 1615u16;
pub static ER_DELAYED_NOT_SUPPORTED: u16 = 1616u16;
pub static WARN_NO_MASTER_INFO: u16 = 1617u16;
pub static WARN_OPTION_IGNORED: u16 = 1618u16;
pub static WARN_PLUGIN_DELETE_BUILTIN: u16 = 1619u16;
pub static WARN_PLUGIN_BUSY: u16 = 1620u16;
pub static ER_VARIABLE_IS_READONLY: u16 = 1621u16;
pub static ER_WARN_ENGINE_TRANSACTION_ROLLBACK: u16 = 1622u16;
pub static ER_SLAVE_HEARTBEAT_FAILURE: u16 = 1623u16;
pub static ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE: u16 = 1624u16;
pub static ER_NDB_REPLICATION_SCHEMA_ERROR: u16 = 1625u16;
pub static ER_CONFLICT_FN_PARSE_ERROR: u16 = 1626u16;
pub static ER_EXCEPTIONS_WRITE_ERROR: u16 = 1627u16;
pub static ER_TOO_LONG_TABLE_COMMENT: u16 = 1628u16;
pub static ER_TOO_LONG_FIELD_COMMENT: u16 = 1629u16;
pub static ER_FUNC_INEXISTENT_NAME_COLLISION: u16 = 1630u16;
pub static ER_DATABASE_NAME: u16 = 1631u16;
pub static ER_TABLE_NAME: u16 = 1632u16;
pub static ER_PARTITION_NAME: u16 = 1633u16;
pub static ER_SUBPARTITION_NAME: u16 = 1634u16;
pub static ER_TEMPORARY_NAME: u16 = 1635u16;
pub static ER_RENAMED_NAME: u16 = 1636u16;
pub static ER_TOO_MANY_CONCURRENT_TRXS: u16 = 1637u16;
pub static WARN_NON_ASCII_SEPARATOR_NOT_IMPLEMENTED: u16 = 1638u16;
pub static ER_DEBUG_SYNC_TIMEOUT: u16 = 1639u16;
pub static ER_DEBUG_SYNC_HIT_LIMIT: u16 = 1640u16;
pub static ER_DUP_SIGNAL_SET: u16 = 1641u16;
pub static ER_SIGNAL_WARN: u16 = 1642u16;
pub static ER_SIGNAL_NOT_FOUND: u16 = 1643u16;
pub static ER_SIGNAL_EXCEPTION: u16 = 1644u16;
pub static ER_RESIGNAL_WITHOUT_ACTIVE_HANDLER: u16 = 1645u16;
pub static ER_SIGNAL_BAD_CONDITION_TYPE: u16 = 1646u16;
pub static WARN_COND_ITEM_TRUNCATED: u16 = 1647u16;
pub static ER_COND_ITEM_TOO_LONG: u16 = 1648u16;
pub static ER_UNKNOWN_LOCALE: u16 = 1649u16;
pub static ER_SLAVE_IGNORE_SERVER_IDS: u16 = 1650u16;
pub static ER_QUERY_CACHE_DISABLED: u16 = 1651u16;
pub static ER_SAME_NAME_PARTITION_FIELD: u16 = 1652u16;
pub static ER_PARTITION_COLUMN_LIST_ERROR: u16 = 1653u16;
pub static ER_WRONG_TYPE_COLUMN_VALUE_ERROR: u16 = 1654u16;
pub static ER_TOO_MANY_PARTITION_FUNC_FIELDS_ERROR: u16 = 1655u16;
pub static ER_MAXVALUE_IN_VALUES_IN: u16 = 1656u16;
pub static ER_TOO_MANY_VALUES_ERROR: u16 = 1657u16;
pub static ER_ROW_SINGLE_PARTITION_FIELD_ERROR: u16 = 1658u16;
pub static ER_FIELD_TYPE_NOT_ALLOWED_AS_PARTITION_FIELD: u16 = 1659u16;
pub static ER_PARTITION_FIELDS_TOO_LONG: u16 = 1660u16;
pub static ER_BINLOG_ROW_ENGINE_AND_STMT_ENGINE: u16 = 1661u16;
pub static ER_BINLOG_ROW_MODE_AND_STMT_ENGINE: u16 = 1662u16;
pub static ER_BINLOG_UNSAFE_AND_STMT_ENGINE: u16 = 1663u16;
pub static ER_BINLOG_ROW_INJECTION_AND_STMT_ENGINE: u16 = 1664u16;
pub static ER_BINLOG_STMT_MODE_AND_ROW_ENGINE: u16 = 1665u16;
pub static ER_BINLOG_ROW_INJECTION_AND_STMT_MODE: u16 = 1666u16;
pub static ER_BINLOG_MULTIPLE_ENGINES_AND_SELF_LOGGING_ENGINE: u16 = 1667u16;
pub static ER_BINLOG_UNSAFE_LIMIT: u16 = 1668u16;
pub static ER_BINLOG_UNSAFE_INSERT_DELAYED: u16 = 1669u16;
pub static ER_BINLOG_UNSAFE_SYSTEM_TABLE: u16 = 1670u16;
pub static ER_BINLOG_UNSAFE_AUTOINC_COLUMNS: u16 = 1671u16;
pub static ER_BINLOG_UNSAFE_UDF: u16 = 1672u16;
pub static ER_BINLOG_UNSAFE_SYSTEM_VARIABLE: u16 = 1673u16;
pub static ER_BINLOG_UNSAFE_SYSTEM_FUNCTION: u16 = 1674u16;
pub static ER_BINLOG_UNSAFE_NONTRANS_AFTER_TRANS: u16 = 1675u16;
pub static ER_MESSAGE_AND_STATEMENT: u16 = 1676u16;
pub static ER_SLAVE_CONVERSION_FAILED: u16 = 1677u16;
pub static ER_SLAVE_CANT_CREATE_CONVERSION: u16 = 1678u16;
pub static ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_BINLOG_FORMAT: u16 = 1679u16;
pub static ER_PATH_LENGTH: u16 = 1680u16;
pub static ER_WARN_DEPRECATED_SYNTAX_NO_REPLACEMENT: u16 = 1681u16;
pub static ER_WRONG_NATIVE_TABLE_STRUCTURE: u16 = 1682u16;
pub static ER_WRONG_PERFSCHEMA_USAGE: u16 = 1683u16;
pub static ER_WARN_I_S_SKIPPED_TABLE: u16 = 1684u16;
pub static ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_BINLOG_DIRECT: u16 = 1685u16;
pub static ER_STORED_FUNCTION_PREVENTS_SWITCH_BINLOG_DIRECT: u16 = 1686u16;
pub static ER_SPATIAL_MUST_HAVE_GEOM_COL: u16 = 1687u16;
pub static ER_TOO_LONG_INDEX_COMMENT: u16 = 1688u16;
pub static ER_LOCK_ABORTED: u16 = 1689u16;
pub static ER_DATA_OUT_OF_RANGE: u16 = 1690u16;
pub static ER_WRONG_SPVAR_TYPE_IN_LIMIT: u16 = 1691u16;
pub static ER_BINLOG_UNSAFE_MULTIPLE_ENGINES_AND_SELF_LOGGING_ENGINE: u16 = 1692u16;
pub static ER_BINLOG_UNSAFE_MIXED_STATEMENT: u16 = 1693u16;
pub static ER_INSIDE_TRANSACTION_PREVENTS_SWITCH_SQL_LOG_BIN: u16 = 1694u16;
pub static ER_STORED_FUNCTION_PREVENTS_SWITCH_SQL_LOG_BIN: u16 = 1695u16;
pub static ER_FAILED_READ_FROM_PAR_FILE: u16 = 1696u16;
pub static ER_VALUES_IS_NOT_INT_TYPE_ERROR: u16 = 1697u16;
pub static ER_ACCESS_DENIED_NO_PASSWORD_ERROR: u16 = 1698u16;
pub static ER_SET_PASSWORD_AUTH_PLUGIN: u16 = 1699u16;
pub static ER_GRANT_PLUGIN_USER_EXISTS: u16 = 1700u16;
pub static ER_TRUNCATE_ILLEGAL_FK: u16 = 1701u16;
pub static ER_PLUGIN_IS_PERMANENT: u16 = 1702u16;
pub static ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE_MIN: u16 = 1703u16;
pub static ER_SLAVE_HEARTBEAT_VALUE_OUT_OF_RANGE_MAX: u16 = 1704u16;
pub static ER_STMT_CACHE_FULL: u16 = 1705u16;
pub static ER_MULTI_UPDATE_KEY_CONFLICT: u16 = 1706u16;
pub static ER_TABLE_NEEDS_REBUILD: u16 = 1707u16;
pub static WARN_OPTION_BELOW_LIMIT: u16 = 1708u16;
pub static ER_INDEX_COLUMN_TOO_LONG: u16 = 1709u16;
pub static ER_ERROR_IN_TRIGGER_BODY: u16 = 1710u16;
pub static ER_ERROR_IN_UNKNOWN_TRIGGER_BODY: u16 = 1711u16;
pub static ER_INDEX_CORRUPT: u16 = 1712u16;
pub static ER_UNDO_RECORD_TOO_BIG: u16 = 1713u16;
pub static ER_BINLOG_UNSAFE_INSERT_IGNORE_SELECT: u16 = 1714u16;
pub static ER_BINLOG_UNSAFE_INSERT_SELECT_UPDATE: u16 = 1715u16;
pub static ER_BINLOG_UNSAFE_REPLACE_SELECT: u16 = 1716u16;
pub static ER_BINLOG_UNSAFE_CREATE_IGNORE_SELECT: u16 = 1717u16;
pub static ER_BINLOG_UNSAFE_CREATE_REPLACE_SELECT: u16 = 1718u16;
pub static ER_BINLOG_UNSAFE_UPDATE_IGNORE: u16 = 1719u16;
pub static ER_PLUGIN_NO_UNINSTALL: u16 = 1720u16;
pub static ER_PLUGIN_NO_INSTALL: u16 = 1721u16;
pub static ER_BINLOG_UNSAFE_WRITE_AUTOINC_SELECT: u16 = 1722u16;
pub static ER_BINLOG_UNSAFE_CREATE_SELECT_AUTOINC: u16 = 1723u16;
pub static ER_BINLOG_UNSAFE_INSERT_TWO_KEYS: u16 = 1724u16;
pub static ER_TABLE_IN_FK_CHECK: u16 = 1725u16;
pub static ER_UNSUPPORTED_ENGINE: u16 = 1726u16;
pub static ER_BINLOG_UNSAFE_AUTOINC_NOT_FIRST: u16 = 1727u16;
