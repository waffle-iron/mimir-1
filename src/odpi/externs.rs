//! ODPI-C externs
use odpi::{flags, opaque, structs};

/// The optional function pointer used in the `ODPISubscrCreateParams` struct.
pub type ODPISubscrCallback =
    Option<unsafe extern "C" fn(context: *mut ::std::os::raw::c_void,
                                message: *mut structs::ODPISubscrMessage)>;

extern "C" {
    pub fn dpiContext_create(majorVersion: ::std::os::raw::c_uint,
                             minorVersion: ::std::os::raw::c_uint,
                             context: *mut *mut opaque::ODPIContext,
                             errorInfo: *mut structs::ODPIErrorInfo)
                             -> ::std::os::raw::c_int;
    pub fn dpiContext_destroy(context: *mut opaque::ODPIContext) -> ::std::os::raw::c_int;
    pub fn dpiContext_getClientVersion(context: *const opaque::ODPIContext,
                                       versionInfo: *mut structs::ODPIVersionInfo)
                                       -> ::std::os::raw::c_int;
    pub fn dpiContext_getError(context: *const opaque::ODPIContext,
                               errorInfo: *mut structs::ODPIErrorInfo);
    pub fn dpiContext_initCommonCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPICommonCreateParams)
                                             -> ::std::os::raw::c_int;
    pub fn dpiContext_initConnCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIConnCreateParams)
                                           -> ::std::os::raw::c_int;
    pub fn dpiContext_initPoolCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIPoolCreateParams)
                                           -> ::std::os::raw::c_int;
    pub fn dpiContext_initSubscrCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPISubscrCreateParams)
                                             -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiConn_addRef(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_beginDistribTrans(conn: *mut opaque::ODPIConn,
                                     formatId: ::std::os::raw::c_long,
                                     transactionId: *const ::std::os::raw::c_char,
                                     transactionIdLength: u32,
                                     branchId: *const ::std::os::raw::c_char,
                                     branchIdLength: u32)
                                     -> ::std::os::raw::c_int;
    pub fn dpiConn_breakExecution(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_changePassword(conn: *mut opaque::ODPIConn,
                                  userName: *const ::std::os::raw::c_char,
                                  userNameLength: u32,
                                  oldPassword: *const ::std::os::raw::c_char,
                                  oldPasswordLength: u32,
                                  newPassword: *const ::std::os::raw::c_char,
                                  newPasswordLength: u32)
                                  -> ::std::os::raw::c_int;
    pub fn dpiConn_close(conn: *mut opaque::ODPIConn,
                         mode: flags::ODPIConnCloseMode,
                         tag: *const ::std::os::raw::c_char,
                         tagLength: u32)
                         -> ::std::os::raw::c_int;
    pub fn dpiConn_commit(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_create(context: *const opaque::ODPIContext,
                          userName: *const ::std::os::raw::c_char,
                          userNameLength: u32,
                          password: *const ::std::os::raw::c_char,
                          passwordLength: u32,
                          connectString: *const ::std::os::raw::c_char,
                          connectStringLength: u32,
                          commonParams: *const structs::ODPICommonCreateParams,
                          createParams: *mut structs::ODPIConnCreateParams,
                          conn: *mut *mut opaque::ODPIConn)
                          -> ::std::os::raw::c_int;
    pub fn dpiConn_deqObject(conn: *mut opaque::ODPIConn,
                             queueName: *const ::std::os::raw::c_char,
                             queueNameLength: u32,
                             options: *mut opaque::ODPIDeqOptions,
                             props: *mut opaque::ODPIMsgProps,
                             payload: *mut opaque::ODPIObject,
                             msgId: *mut *const ::std::os::raw::c_char,
                             msgIdLength: *mut u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_enqObject(conn: *mut opaque::ODPIConn,
                             queueName: *const ::std::os::raw::c_char,
                             queueNameLength: u32,
                             options: *mut opaque::ODPIEnqOptions,
                             props: *mut opaque::ODPIMsgProps,
                             payload: *mut opaque::ODPIObject,
                             msgId: *mut *const ::std::os::raw::c_char,
                             msgIdLength: *mut u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_getCurrentSchema(conn: *mut opaque::ODPIConn,
                                    value: *mut *const ::std::os::raw::c_char,
                                    valueLength: *mut u32)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_getEdition(conn: *mut opaque::ODPIConn,
                              value: *mut *const ::std::os::raw::c_char,
                              valueLength: *mut u32)
                              -> ::std::os::raw::c_int;
    pub fn dpiConn_getEncodingInfo(conn: *mut opaque::ODPIConn,
                                   info: *mut structs::ODPIEncodingInfo)
                                   -> ::std::os::raw::c_int;
    pub fn dpiConn_getExternalName(conn: *mut opaque::ODPIConn,
                                   value: *mut *const ::std::os::raw::c_char,
                                   valueLength: *mut u32)
                                   -> ::std::os::raw::c_int;
    #[allow(dead_code)]
    pub fn dpiConn_getHandle(conn: *mut opaque::ODPIConn,
                             handle: *mut *mut ::std::os::raw::c_void)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_getInternalName(conn: *mut opaque::ODPIConn,
                                   value: *mut *const ::std::os::raw::c_char,
                                   valueLength: *mut u32)
                                   -> ::std::os::raw::c_int;
    pub fn dpiConn_getLTXID(conn: *mut opaque::ODPIConn,
                            value: *mut *const ::std::os::raw::c_char,
                            valueLength: *mut u32)
                            -> ::std::os::raw::c_int;
    pub fn dpiConn_getObjectType(conn: *mut opaque::ODPIConn,
                                 name: *const ::std::os::raw::c_char,
                                 nameLength: u32,
                                 objType: *mut *mut opaque::ODPIObjectType)
                                 -> ::std::os::raw::c_int;
    pub fn dpiConn_getServerVersion(conn: *mut opaque::ODPIConn,
                                    releaseString: *mut *const ::std::os::raw::c_char,
                                    releaseStringLength: *mut u32,
                                    versionInfo: *mut structs::ODPIVersionInfo)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_getStmtCacheSize(conn: *mut opaque::ODPIConn,
                                    cacheSize: *mut u32)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_newDeqOptions(conn: *mut opaque::ODPIConn,
                                 options: *mut *mut opaque::ODPIDeqOptions)
                                 -> ::std::os::raw::c_int;
    pub fn dpiConn_newEnqOptions(conn: *mut opaque::ODPIConn,
                                 options: *mut *mut opaque::ODPIEnqOptions)
                                 -> ::std::os::raw::c_int;
    pub fn dpiConn_newMsgProps(conn: *mut opaque::ODPIConn,
                               props: *mut *mut opaque::ODPIMsgProps)
                               -> ::std::os::raw::c_int;
    pub fn dpiConn_newSubscription(conn: *mut opaque::ODPIConn,
                                   params: *mut structs::ODPISubscrCreateParams,
                                   subscr: *mut *mut opaque::ODPISubscr,
                                   subscrId: *mut u32)
                                   -> ::std::os::raw::c_int;
    pub fn dpiConn_newTempLob(conn: *mut opaque::ODPIConn,
                              lobType: flags::ODPIOracleTypeNum,
                              lob: *mut *mut opaque::ODPILob)
                              -> ::std::os::raw::c_int;
    pub fn dpiConn_newVar(conn: *mut opaque::ODPIConn,
                          oracleTypeNum: flags::ODPIOracleTypeNum,
                          nativeTypeNum: flags::ODPINativeTypeNum,
                          maxArraySize: u32,
                          size: u32,
                          sizeIsBytes: ::std::os::raw::c_int,
                          isArray: ::std::os::raw::c_int,
                          objType: *mut opaque::ODPIObjectType,
                          var: *mut *mut opaque::ODPIVar,
                          data: *mut *mut structs::ODPIData)
                          -> ::std::os::raw::c_int;
    pub fn dpiConn_ping(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_prepareDistribTrans(conn: *mut opaque::ODPIConn,
                                       commitNeeded: *mut ::std::os::raw::c_int)
                                       -> ::std::os::raw::c_int;
    pub fn dpiConn_prepareStmt(conn: *mut opaque::ODPIConn,
                               scrollable: ::std::os::raw::c_int,
                               sql: *const ::std::os::raw::c_char,
                               sqlLength: u32,
                               tag: *const ::std::os::raw::c_char,
                               tagLength: u32,
                               stmt: *mut *mut opaque::ODPIStmt)
                               -> ::std::os::raw::c_int;
    pub fn dpiConn_release(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_rollback(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_setAction(conn: *mut opaque::ODPIConn,
                             value: *const ::std::os::raw::c_char,
                             valueLength: u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_setClientIdentifier(conn: *mut opaque::ODPIConn,
                                       value: *const ::std::os::raw::c_char,
                                       valueLength: u32)
                                       -> ::std::os::raw::c_int;
    pub fn dpiConn_setClientInfo(conn: *mut opaque::ODPIConn,
                                 value: *const ::std::os::raw::c_char,
                                 valueLength: u32)
                                 -> ::std::os::raw::c_int;
    pub fn dpiConn_setCurrentSchema(conn: *mut opaque::ODPIConn,
                                    value: *const ::std::os::raw::c_char,
                                    valueLength: u32)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_setDbOp(conn: *mut opaque::ODPIConn,
                           value: *const ::std::os::raw::c_char,
                           valueLength: u32)
                           -> ::std::os::raw::c_int;
    pub fn dpiConn_setExternalName(conn: *mut opaque::ODPIConn,
                                   value: *const ::std::os::raw::c_char,
                                   valueLength: u32)
                                   -> ::std::os::raw::c_int;
    pub fn dpiConn_setInternalName(conn: *mut opaque::ODPIConn,
                                   value: *const ::std::os::raw::c_char,
                                   valueLength: u32)
                                   -> ::std::os::raw::c_int;
    pub fn dpiConn_setModule(conn: *mut opaque::ODPIConn,
                             value: *const ::std::os::raw::c_char,
                             valueLength: u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_setStmtCacheSize(conn: *mut opaque::ODPIConn,
                                    cacheSize: u32)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_shutdownDatabase(conn: *mut opaque::ODPIConn,
                                    mode: flags::ODPIShutdownMode)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_startupDatabase(conn: *mut opaque::ODPIConn,
                                   mode: flags::ODPIStartupMode)
                                   -> ::std::os::raw::c_int;
}

#[allow(dead_code)]
extern "C" {
    pub fn dpiData_getBytes(data: *mut structs::ODPIData) -> *mut structs::ODPIBytes;
    pub fn dpiData_getDouble(data: *mut structs::ODPIData) -> f64;
    pub fn dpiData_setBytes(data: *mut structs::ODPIData,
                            ptr: *mut ::std::os::raw::c_char,
                            length: u32);
    pub fn dpiData_setInt64(data: *mut structs::ODPIData, value: i64);
}

extern "C" {
    pub fn dpiDeqOptions_getMode(options: *mut opaque::ODPIDeqOptions,
                                 value: *mut flags::ODPIDeqMode)
                                 -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiEnqOptions_getVisibility(options: *mut opaque::ODPIEnqOptions,
                                       value: *mut flags::ODPIVisibility)
                                       -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiLob_getChunkSize(lob: *mut opaque::ODPILob, size: *mut u32) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiMsgProps_getDeliveryMode(props: *mut opaque::ODPIMsgProps,
                                       value: *mut flags::ODPIMessageDeliveryMode)
                                       -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiObjectType_addRef(objType: *mut opaque::ODPIObjectType) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiStmt_addRef(stmt: *mut opaque::ODPIStmt) -> ::std::os::raw::c_int;
    pub fn dpiStmt_bindByName(stmt: *mut opaque::ODPIStmt,
                              name: *const ::std::os::raw::c_char,
                              nameLength: u32,
                              var: *mut opaque::ODPIVar)
                              -> ::std::os::raw::c_int;
    pub fn dpiStmt_bindByPos(stmt: *mut opaque::ODPIStmt,
                             pos: u32,
                             var: *mut opaque::ODPIVar)
                             -> ::std::os::raw::c_int;
    pub fn dpiStmt_bindValueByName(stmt: *mut opaque::ODPIStmt,
                                   name: *const ::std::os::raw::c_char,
                                   nameLength: u32,
                                   nativeTypeNum: flags::ODPINativeTypeNum,
                                   data: *mut structs::ODPIData)
                                   -> ::std::os::raw::c_int;
    pub fn dpiStmt_bindValueByPos(stmt: *mut opaque::ODPIStmt,
                                  pos: u32,
                                  nativeTypeNum: flags::ODPINativeTypeNum,
                                  data: *mut structs::ODPIData)
                                  -> ::std::os::raw::c_int;
    pub fn dpiStmt_close(stmt: *mut opaque::ODPIStmt,
                         tag: *const ::std::os::raw::c_char,
                         tagLength: u32)
                         -> ::std::os::raw::c_int;
    pub fn dpiStmt_execute(stmt: *mut opaque::ODPIStmt,
                           mode: flags::ODPIExecMode,
                           numQueryColumns: *mut u32)
                           -> ::std::os::raw::c_int;
    pub fn dpiStmt_executeMany(stmt: *mut opaque::ODPIStmt,
                               mode: flags::ODPIExecMode,
                               numIters: u32)
                               -> ::std::os::raw::c_int;
    pub fn dpiStmt_fetch(stmt: *mut opaque::ODPIStmt,
                         found: *mut ::std::os::raw::c_int,
                         bufferRowIndex: *mut u32)
                         -> ::std::os::raw::c_int;
    pub fn dpiStmt_fetchRows(stmt: *mut opaque::ODPIStmt,
                             maxRows: u32,
                             bufferRowIndex: *mut u32,
                             numRowsFetched: *mut u32,
                             moreRows: *mut ::std::os::raw::c_int)
                             -> ::std::os::raw::c_int;
    pub fn dpiStmt_getBatchErrorCount(stmt: *mut opaque::ODPIStmt,
                                      count: *mut u32)
                                      -> ::std::os::raw::c_int;
    pub fn dpiStmt_getBatchErrors(stmt: *mut opaque::ODPIStmt,
                                  numErrors: u32,
                                  errors: *mut structs::ODPIErrorInfo)
                                  -> ::std::os::raw::c_int;
    pub fn dpiStmt_getBindCount(stmt: *mut opaque::ODPIStmt,
                                count: *mut u32)
                                -> ::std::os::raw::c_int;
    pub fn dpiStmt_getBindNames(stmt: *mut opaque::ODPIStmt,
                                numBindNames: u32,
                                bindNames: *mut *const ::std::os::raw::c_char,
                                bindNameLengths: *mut u32)
                                -> ::std::os::raw::c_int;
    pub fn dpiStmt_getFetchArraySize(stmt: *mut opaque::ODPIStmt,
                                     arraySize: *mut u32)
                                     -> ::std::os::raw::c_int;
    pub fn dpiStmt_getInfo(stmt: *mut opaque::ODPIStmt,
                           info: *mut structs::ODPIStmtInfo)
                           -> ::std::os::raw::c_int;
    pub fn dpiStmt_getQueryInfo(stmt: *mut opaque::ODPIStmt,
                                pos: u32,
                                info: *mut structs::ODPIQueryInfo)
                                -> ::std::os::raw::c_int;
    pub fn dpiStmt_getQueryValue(stmt: *mut opaque::ODPIStmt,
                                 pos: u32,
                                 nativeTypeNum: *mut ::std::os::raw::c_int,
                                 data: *mut *mut structs::ODPIData)
                                 -> ::std::os::raw::c_int;
    pub fn dpiStmt_release(stmt: *mut opaque::ODPIStmt) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiSubscr_addRef(subscr: *mut opaque::ODPISubscr) -> ::std::os::raw::c_int;
}

#[allow(dead_code)]
extern "C" {
    pub fn dpiVar_addRef(var: *mut opaque::ODPIVar) -> ::std::os::raw::c_int;
    pub fn dpiVar_copyData(var: *mut opaque::ODPIVar,
                           pos: u32,
                           sourceVar: *mut opaque::ODPIVar,
                           sourcePos: u32)
                           -> ::std::os::raw::c_int;
    pub fn dpiVar_getData(var: *mut opaque::ODPIVar,
                          numElements: *mut u32,
                          data: *mut *mut structs::ODPIData)
                          -> ::std::os::raw::c_int;
    pub fn dpiVar_getNumElementsInArray(var: *mut opaque::ODPIVar,
                                        numElements: *mut u32)
                                        -> ::std::os::raw::c_int;
    pub fn dpiVar_getSizeInBytes(var: *mut opaque::ODPIVar,
                                 sizeInBytes: *mut u32)
                                 -> ::std::os::raw::c_int;
    pub fn dpiVar_release(var: *mut opaque::ODPIVar) -> ::std::os::raw::c_int;
    pub fn dpiVar_setFromBytes(var: *mut opaque::ODPIVar,
                               pos: u32,
                               value: *const ::std::os::raw::c_char,
                               valueLength: u32)
                               -> ::std::os::raw::c_int;
}
