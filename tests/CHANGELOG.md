## Unreleased

- PR#1278: Mojo debug. Ability to get all interims columns / Vladimir
- [BUILD] Next devel version 2.5.17-SNAPSHOT / Vladimir Ovsyannikov

## 2.5.16 - 2021-04-16

- activate test for issue #1218 / Petr Kozelka
- PR#1276: minimized use of MojoTransformMeta / Petr Kozelka
- PR#1277: Use already built versions of protobuf and sentencepiece from dai-third-party / Vladimir
- suppress JMH benchmarks as there is currently none / Petr Kozelka
- PR#1229: refactor(MojoTransform): unify builder and its instance / Petr Kozelka
- fix R pkg / Vladimir Ovsyannikov
- use prev version of pkgload / Vladimir Ovsyannikov
- tiny rework after review / Vladimir Ovsyannikov
- ignore nodes wo avx / Vladimir Ovsyannikov
- rework after review / Vladimir Ovsyannikov
- fix permission / Vladimir Ovsyannikov
- fix passing by ref / Vladimir Ovsyannikov
- pass by reference / Vladimir Ovsyannikov
- ignore mr-0xe1 / Vladimir Ovsyannikov
- docker configuration / Vladimir Ovsyannikov
- fix xception model / Vladimir Ovsyannikov
- use existing test data / Vladimir Ovsyannikov
- fix comment / Vladimir Ovsyannikov
- add shap test / Vladimir Ovsyannikov
- fix img test / Vladimir Ovsyannikov
- fix test / Vladimir Ovsyannikov
- fix LG_WARN error / Vladimir Ovsyannikov
- fix var name / Vladimir Ovsyannikov
- fix tests, prevent infinite loop / Vladimir Ovsyannikov
- get SHAP feature names from computationl graph itself / Vladimir Ovsyannikov
- add group into visualization script / Vladimir Ovsyannikov
- ShapTest: added test for #1218 showing that java runtime does not have this bug / Petr Kozelka
- issue #1218: added data from Martin Barus to reproduce the problem / Petr Kozelka
- PR#1260: refactor(TreeShap): minor code changes for logical consistency and less memory allocations / Petr Kozelka
- [BUILD] Next devel version 2.5.16-SNAPSHOT / mmalohlava

## 2.5.15 - 2021-04-01

- PR#1264: fix(decision trees): unify comparison between SHAP and predict / Petr Kozelka
- PR#1262: [TEST] Test to support https://github.com/h2oai/mojo2/pull/1254 / Michal Malohlava
- PR#1261: cleanup(TOML): removed TomlWriter code that is long long unused / Petr Kozelka
- PR#1259: fix(log): warn about prematurely stopping traversal only once, and mention number of occurrences / Petr Kozelka
- [BUILD] Next devel version 2.5.15-SNAPSHOT / mmalohlava

## 2.5.14 - 2021-03-30

- PR#1257: Revertign previous changes. / Michal Malohlava
- PR#1223: Include libz for r / Joe
- [BUILD] Next devel version 2.5.14-SNAPSHOT / mmalohlava

## 2.5.13 - 2021-03-30

- PR#1255: warn about missing Pipeline.dai_version, or report it on debug level / Petr Kozelka
- PR#1254: fix(SHAP): no longer prevent SHAP based on tree data (dataCount==0) / Petr Kozelka
- PR#1250: fix(SHAP): collection of SHAP fixes and refactors / Petr Kozelka
- don't use nodes wo avx / Vladimir Ovsyannikov
- another workaround to fix permissions / Vladimir Ovsyannikov
- PR#1251: rollback wrong changes / Vladimir
- fix permission / Vladimir Ovsyannikov
- fix permission / Vladimir Ovsyannikov
- fix docker image: download cryptopp820 from s3 / Vladimir Ovsyannikov
- download cryptopp820 from s3 / Vladimir Ovsyannikov
- fix(CsvAssert): a bug in test that caused it to sometimes tolerate huge differences / Petr Kozelka
- PR#1248: chore(parser): improve parser error reporting. / Michal Malohlava
- PR#1249: Fix docker permission / Vladimir
- PR#1245: fix(ShapWeightLocator): improved lookup of blending weights / Petr Kozelka
- CapnpMojoModelReader: added missing condition for a log message / Petr Kozelka
- fix(MojoPipelineProtoImpl): typo in logger category / Petr Kozelka
- PR#1241: fix(SHAP): assign correct classlabels / Petr Kozelka
- PR#1191: feat: comparable tracking output / Petr Kozelka
- PR#1239: chore(Jenkinsfile): remove incorrect use of try-finally / Petr Kozelka
- PR#1235: [BUILD] Release 2.5.12 / Petr Kozelka
- PR#1234: cleanup in builders: no need for setTransformBuilder / Petr Kozelka
- PR#1231: fix: never lookup columns by name anymore / Petr Kozelka
- PR#1233: chore(Jenkinsfile): make stage naming easier to distinguish / Petr Kozelka
- PR#1217: cleanup in boosters / Petr Kozelka
- PR#1230: refactor: simplify transforms / Petr Kozelka
- PR#1228: refactor(javassist): unify transform and its builder class for generated cases by adding JavassistMojoTransform as the base class / Petr Kozelka
- PR#1226: fix  #21526 daimojo segfaults if model loaded twice / Vladimir
- PR#1222: moment: compute without javassist, and heavily simplified the algorithm / Petr Kozelka
- PR#1221: impute: compute without javassist, not even using reflections / Petr Kozelka
- PR#1220: refactor(MojoTransformBuilder): do not use non-array constructors / Petr Kozelka
- PR#1216: added some missing option / Petr Kozelka
- [BUILD] Next devel version 2.5.12-SNAPSHOT / Vladimir Ovsyannikov

## 2.5.11 - 2021-03-02

- PR#1210: rename: shortened GLM test directories / Petr Kozelka
- PR#1214: [Docs] Typo fix / Zain
- PR#1212: Report error when feature names don't match / Vladimir
- PR#1188: Upgrade to TF 1.15.5 (latest 1.x release) / Arno Candel
- 🚨 Hotfix of syntax in PATCH_VERSION.txt / mmalohlava
- [BUILD] Next devel version 2.5.11-SNAPSHOT / mmalohlava

## 2.5.10 - 2021-02-09

- [DOCS] how to release mojo2 runtime. / mmalohlava
- PR#1209: Fix snyk.io security problems / Michal Malohlava
- PR#1198: feat(SHAP): implemented for linear boosters / Petr Kozelka
- PR#1207: fix(cpp/marshaller): store base_score into pipeline / Petr Kozelka
- PR#1205: fix(MojoPipelineProtoImpl): removed the 'break' statement that could hurt recursive traversal / Petr Kozelka
- PR#1204: fix(MojoPipelineMeta.consistencyChecks): opposite condition / Petr Kozelka
- PR#1190: [ARTIFACTS] For fatjar distribution: relocate javassist. / Michal Malohlava
- PR#1175: refactor(MojoPipelineProtoImpl): simplify data model for nested transformations / Petr Kozelka
- [ITEST] Missing exception for 1.9.0 branch / mmalohlava
- [ITEST] Adding missing migration tests / mmalohlava
- PR#1196: fix: don't fail when there is an unnamed column in input CSV / Petr Kozelka
- PR#1195: fix: report invalid "transformed" items using warning, not error (log level) / Petr Kozelka
- PR#1168: Unify CI pipeline / Vladimir
- PR#1184: refactor(MojoTransformBuilder): minimize use of non-array constructors / Petr Kozelka
- PR#1180: chore(Jenkinsfile): enforce against weird behavior in gradle/jenkins/docker / Petr Kozelka
- [BUILD] Next devel version 2.5.10-SNAPSHOT / mmalohlava

## 2.5.9 - 2021-01-26

- PR#1185: [HOTFIX] Remove race from our jenkins jobs. / Michal Malohlava
- PR#1178: [cpp] Implementing common date parser with support of '%f' / Michal Malohlava
- PR#1182: [HOTFIX] 🚒 Fix build in master / Michal Malohlava
- PR#1179: Revert "cleanup in PipelineLoader: removed getInput, getOutput" / Arno Candel
- PR#1177: refactor: pipeline transfomer readers switched to non-static / Petr Kozelka
- PR#1176: chore(MojoTransformExecPipeBuilder): removed dead and commented code / Petr Kozelka
- PR#1174: MojoTransformExecPipeBuilder: merge with its instance / Petr Kozelka
- PR#1062: feat: SHAP for ensembles / Petr Kozelka
- PR#1173: [BUG][java] Fix of NPE in StopWatch / Michal Malohlava
- Next devel version 2.5.9-SNAPSHOT / mmalohlava

## 2.5.8 - 2021-01-08

- [HOTFIX] Fix a syntax error. / mmalohlava
- PR#1169: [java] Null representation for StringCat transformer. / Michal Malohlava
- PR#1165: [java] relaxing log transformer assert / Michal Malohlava
- PR#1162: Initial PR to make mojo easy build-able without docker / Vladimir
- PR#1164: Add dai_grant_version logic to support DAI itself making a scoring license for the MOJO. / Tom Kraljevic
- PR#1159: ReaderBackend: Avoid duplicate parsing and allocation / Petr Kozelka
- [cpp] License fix / mmalohlava
- Next devel version 2.5.8-SNAPSHOT / mmalohlava

## 2.5.7 - 2020-12-28

- PR#1094: [java][h2o3] Enable TE in H2O-3 model transformer / Michal Malohlava
- [BUILD] Publish fatjar into Maven central / mmalohlava
- PR#1156: MojoTransform changed into interface / Petr Kozelka
- PR#1157: MojoColumn: removed Kind.Response, long after its deprecation period expired / Petr Kozelka
- PR#1146: Deduplicate code for reading protobuf arrays / Petr Kozelka
- PR#1145: Enhance boolean parser with float-like values "1.0" and "0.0" / Petr Kozelka
- PR#1138: Reimplement H2O-3 pipelines as transformations / Petr Kozelka
- [BUILD] Next dev version 2.5.7-SNAPSHOT / mmalohlava

## 2.5.6 - 2020-12-10

- PR#1125: Added logging with slf4j / Petr Kozelka
- PR#1142: [TREE] Fix missing cast to computation type (float in this case) / Michal Malohlava
- PR#1143: fix: explicitly set XGB splittype to LT / Petr Kozelka
- PR#1140: [BUGFIX] Correct behavior of AsType in case INT64 to Date transformation. / Michal Malohlava
- [BUILD] Next dev version 2.5.6-SNAPSHOT / mmalohlava

## 2.5.5 - 2020-12-02

- PR#1135: hotfix for h2oai issue #19515 / Petr Kozelka
- PR#1134: Minor cleanup / Petr Kozelka
- PR#1131: removed default for "inputCsvFilename", causing regression in CLI behavior since 2.5.1 / Petr Kozelka
- PR#1132: NewColumnOp: removed code for unsupported types / Petr Kozelka
- PR#1115: [TEST] Using pipeline with explicit OrigTransformer for Iris test. / Michal Malohlava
- PR#1128: cpp: support bool as output type in daimojo / Michal Malohlava
- [BUILD] Next dev version 2.5.5-SNAPSHOT / mmalohlava

## 2.5.4 - 2020-11-12

- PR#1120: [PARSER] Option to trim whitespaces for parsed fields. / Michal Malohlava
- PR#1122: [BUILD] Cache C++ build images in harbor.h2o.ai / Michal Malohlava
- PR#1121: [BUILD] Use local nexus to fetch artifacts to avoid timeouts from Maven / Michal Malohlava
- PR#1111: [CI] parallelize cpp build / Michal Malohlava
- MojoPipelineProtoImpl: remove unnecessary param and field `transforms` / Petr Kozelka
- PR#1116: MojoPipelineMeta: moved out of its container class / Petr Kozelka
- PR#1110: minor code improvements to minimize SHAP PR / Petr Kozelka
- [DEV] Next dev version 2.5.4-SNAPSHOT / mmalohlava

## 2.5.3 - 2020-10-27

- PR#1108: [CI] Fix releasing via tag. / Michal Malohlava
- [DEV] Next dev version 2.5.4-SNAPSHOT / mmalohlava

## 2.5.3 - 2020-10-27 [YANKED]

- PR#1106: [BUILD] Make release only on a tag. / Michal Malohlava
- PR#1100: protobuf: added identity op / Petr Kozelka
- PR#1105: [ITEST] Disable test_mojo_twosigma_1 test due to problem detected. / Michal Malohlava
- PR#1076: Add MOJO for StringConcat / Arno Candel
- PR#1101: [ITEST] Regression testing for DAI 1.8.9 and 1.9.0.4 / Michal Malohlava
- PR#1098: [HOTFIX] 🚒 Fix the wrong usage of tree limit. / Michal Malohlava
- PR#1097: [ITEST] Add DAI 1.8.8, 1.9.0.2, 1.9.0.3 versions / Michal Malohlava
- PR#1095: Changed assert into IllegalStateException when there is no output column / Petr Kozelka
- PR#1088: Changed local build output to mojo2-runtime.jar / Petr Kozelka
- PR#1091: Fixed: K-Means is not a model / Petr Kozelka
- CsvComparingBatchHandler: improvemed report notation / Petr Kozelka
- MojoFrameBuilder: better info in exception message / Petr Kozelka
- PipelineViewerTest: improved notation of transforms / Petr Kozelka
- PR#1089: protobuf: wrap all license fields in separate License structure / Petr Kozelka
- PR#1085: [CUSTOM OP] Enforce naming convention for custom transformers. / Michal Malohlava
- PR#1087: Added Pipeline.license_serial_number to hold serial number of the DAI license key, and other fields related to license / Petr Kozelka
- PR#1054: SHAP: Fail on non-SHAP operation in pipeline / Petr Kozelka
- PR#1084: TreeShap: minor refactors / Petr Kozelka
- [DEVEL] Next development version / mmalohlava
- MojoFrameBuilder: improved exception message / Petr Kozelka
- MojoTransformationGroup: implement proper hashCode, equals and toString / Petr Kozelka

## 2.5.2 - 2020-09-18

- PR#1070: [ITEST] Add more integration tests. / Michal Malohlava
- PR#1082: [CUSTOM] Custom h2o-3 transformer (encapsulates supervised models) / Michal Malohlava
- PR#1079: Wonng setup for keep class setup for opencsv package. / Michal Malohlava
- PR#1081: [CPP] Debugging utility. / Michal Malohlava
- PR#1075: TreeShap: fixed bug in conversion from C++ code / Petr Kozelka
- PR#1074: fixed bug in counting totalDataRows / Petr Kozelka

## 2.5.1 - 2020-09-09

- PR#1072: Tree models: make featureMap optional param / Petr Kozelka
- PR#1073: [cpp] disable tree shap computing when tree model doesn't use input features / Qiang Kou (KK)
- PR#1067: [h2o3] Upgrade h2o-genmodel to 3.30.1.2 / Michal Malohlava
- TreeBooster: fixed little bug preventing to use source columns of type double (Float64) / Petr Kozelka
- PR#1064: Initial module with ExecuteMojo utility. / Michal Malohlava
- PR#1065: Compatibility fixes for version 2.5.0 / Petr Kozelka
- bugfix in MojoFrameMeta: use LinkedHasMap to ensure keeping order in getColumnNames() / Petr Kozelka

## 2.5.0 - 2020-08-28

- PR#1063: Parse and expose transformation group in MojoTransformMeta / Petr Kozelka
- PR#1061: Expose columns in PipelineLoader / Petr Kozelka
- PR#1060: Unify tree-shaking optimization in protocol-agnostic manner / Petr Kozelka
- PR#1058: Use Java collections to represent lists of columns / Petr Kozelka
- implemented simple example of pipeline viewer using loader api / Petr Kozelka
- PR#1057: [cpp] trimming whitespace before roberta tokenizing / Qiang Kou (KK)
- PR#1056: Minor cleanups in mojoframe classes / Petr Kozelka
- PR#1055: Properly split methods from ArrayUtils between api and impl modules / Petr Kozelka
- PR#1053: Cleanup: minor post-shap fixes and improvements / Petr Kozelka
- PR#971: Design and implement SHAP feature / Petr Kozelka
- PR#1051: [cpp] trimming whitespace in license string / Qiang Kou (KK)
- Make feature list more predictable / Petr Kozelka
- minor test adjustments / Petr Kozelka
- BigTestForDirs improvement: show used transforms / Petr Kozelka
- PR#1050: Fail when there is no featureMap param in Model / Petr Kozelka
- PR#1048, #1047: [c++] support bool in runtime / Qiang Kou (KK)
- PR#1046: Minor cleanups / Petr Kozelka

## 2.4.10 - 2020-08-10

- [DOC] add download links for PPC and OSX. close #1020 / Qiang Kou
- PR#1043: [BUILD] use pre-built opencv / Qiang Kou (KK)
- PR#1044: Minor cleanup in MojoColumn / Petr Kozelka
- PR#1037: Minor cleanups and protobuf adjustments / Petr Kozelka
- PR#1040: [bulid] update links in download page / Qiang Kou (KK)
- PR#1039: [Java] removed junk code from LinearBooster / Petr Kozelka
- PR#1035: [doc] build download page / Qiang Kou (KK)
- PR#1038: resolved "warning: variable 'status' set but not used [-Wunused-but-set-variable]" / Petr Kozelka
- fixed typo in cli docs / Petr Kozelka
- PR#1036: [build] make sure docker is not mount as root / Qiang Kou (KK)
- PR#1034: MojoTransformSigmoidBuilder: fixed naming of generated class / Petr Kozelka
- PR#1033: Enhanced Sigmoid operation to support Float32 type / Petr Kozelka
- PR#1031: Enhance Softmax operation with support for Float32 / Petr Kozelka
- PR#1032: build for py38 / Qiang Kou (KK)
- bpe result in roberta can be unknown in the vocabulary / Qiang Kou

## 2.4.9 - 2020-07-27

- PR#1029: [BUILD] enable PPC build / Qiang Kou (KK)
- PR#1028: [cpp] fix libdaimojo / Qiang Kou (KK)
- PR#1027: [BUILD] build python wheel for multiple python versions on Linux / Qiang Kou (KK)
- PR#1026: [cpp] image transformer should reture None if image path is empty / Qiang Kou (KK)
- PR#1025: [py] remove -stdlib=libc++ from linking flag / Qiang Kou (KK)
- [BUILD] disable PPC build for now. / Qiang Kou
- PR#1024: [cpp] use libcryptopp for license checking / Qiang Kou (KK)
- [BUILD] bulid OSX pkg first, so we won't be blocked by PPC machines / Qiang Kou

## 2.4.8 - 2020-07-19

- PR#1018: disable vsx on powerpc / Qiang Kou (KK)
- PR#1016: [cpp] using libjpeg-turbo for loading jpeg images. / Qiang Kou (KK)
- PR#1006: Updated test_mojo_compatibility.py file to collect timings for each t… / Jasmine Parekh
- PR#1015, #1014: [cpp] use INTER_NEAREST in image resizing / Qiang Kou (KK)

## 2.4.7 - 2020-07-08

- PR#1013: [cpp] link to libjpeg for loading images / Qiang Kou (KK)
- PR#1012: [cpp] fix string hasher in ftrl model / Qiang Kou (KK)
- PR#1010: Documentation for commandline options / Petr Kozelka

## 2.4.6 - 2020-07-04

- PR#1009: [cpp] no in-place modification in mojo runtime / Qiang Kou (KK)
- PR#1005: [cpp] switch to opencv for image resizing operation / Qiang Kou (KK)
- PR#1004: Introduced BatchedCsvReader for convenient batched CSV reading / Petr Kozelka

## 2.4.5 - 2020-06-24

- PR#999: [DEPS] Replace the private KLime dependency with the latest version of OSS H2O-3 GenModel. / Michal Malohlava
- PR#997: [MOJO2-992] Fix parser bug when the categorical column is listed as the last column. / Michal Malohlava
- PR#1003, #1002: only pick CLS token in bert transformer / Qiang Kou (KK)
- PR#1000: [ITEST] Add DAI v187 state. / Michal Malohlava
- PR#996, #995: [cpp] trim text before tokenizing / Qiang Kou (KK)
- PR#994, #993: fix order of attention_mask and token_type_ids / Qiang Kou (KK)
- Revert "[RELEASE] Temporarily remove KLime from build process. (#990)" / Petr Kozelka

## 2.4.4 - 2020-06-15 [YANKED]

- PR#990: [RELEASE] Temporarily remove KLime from build process. / Qiang Kou (KK)
- PR#986: Remove CapnpWriter and related obsolete code / Petr Kozelka
- PR#987: [py] package libtorch in whl for osx / Qiang Kou (KK)
- PR#984: [ITEST] Backward compatibility state for 1.8.6 and 1.8.6.3 / Michal Malohlava
- PR#559: [IDL] add numeric statistics info from training set / Qiang Kou (KK)

## 2.4.3 - 2020-06-08 [YANKED]

- PR#981: Cleanup in tree-related code / Petr Kozelka
- PR#983, #982: [cpp] base margin in linear regression model is missing / Qiang Kou (KK)
- PR#980, #975: [cpp] image might have different num of channels / Qiang Kou (KK)
- PR#979, #977: [py] not throw exception from daimojo package / Qiang Kou (KK)
- PR#978, #976: [cpp] add caffe mode in imagenet preprocessing / Qiang Kou (KK)
- PR#974: Revert "[RELEASE] Temporarily remove KLime from build process. (#973)" / Navdeep Gill
- [VERSION] Fix patch specification. / mmalohlava
- PR#973: [RELEASE] Temporarily remove KLime from build process. / Michal Malohlava
- PR#972: [cpp] make CalculateContributions template with T / Qiang Kou (KK)
- PR#932: Lightweight pipeline loading / Petr Kozelka
- PR#969, #953: [cpp] move u_cleanup into deconstructor of pipeline / Qiang Kou (KK)
- PR#968: [cpp] bundle libtorch in daimojo pkg / Qiang Kou (KK)
- PR#967: [cpp] add test for xception model / Qiang Kou (KK)
- PR#966: [cpp] clean up the bert tokenizer implementatoin / Qiang Kou (KK)
- PR#965: minor code cleanup in MojoFrameMeta: / Petr Kozelka

## 2.4.2 - 2020-05-14

- PR#964: [cpp] fix image pixel value manipulation  / Qiang Kou (KK)
- PR#962: [cpp] add simple test for image processing / Qiang Kou (KK)
- PR#963: [py] add get_pipeline_transformer_size into daimojo pkg / Qiang Kou (KK)
- PR#961: [cpp] refactor tf scorer to handle imaging data / Qiang Kou (KK)
- PR#957: CSV processing: support selected input columns on output / Petr Kozelka
- PR#960: [cpp] add ImageNet preprocessing transformer / Qiang Kou (KK)
- PR#959: [cpp] image resizing transformer / Qiang Kou (KK)
- build.gradle: remove trailing whitespaces / Petr Kozelka
- PR#956: cleanup: remove SimpleMojoBackend, it's not used anymore / Petr Kozelka
- PR#955: [cpp] add image data type / Qiang Kou (KK)
- PR#954: [cpp] add roberta base tokenizer / Qiang Kou (KK)
- PR#949: Cleanup in MojoPipelineProtoImpl / Petr Kozelka
- PR#950: Implemented H2O3BackendAdapter for easier integration into pipeline providers / Petr Kozelka
- PR#952: [cpp] add xlm roberta tokenizer / Qiang Kou (KK)
- PR#921: Decouple reader backends from pipeline format / Petr Kozelka
- PR#947: [cpp] add xlnet tokenizer / Qiang Kou (KK)
- PR#945: Performance improvements and cleanup in H2O-3 pipeline implementations / Petr Kozelka

## 2.4.1 - 2020-04-27

- PR#946: [cpp] add camembert tokenizer / Qiang Kou (KK)
- PR#944: Remove dead code from MojoFrameMetaBuilder / Petr Kozelka
- PR#906: issue 888: csv with unescaped newline - not supported right now / Petr Kozelka
- PR#943: [cpp] update packaging docker image / Qiang Kou (KK)
- PR#942: [cpp] add albert tokenizer / Qiang Kou (KK)
- PR#939: Jenkinsfile: added !micro, to avoid pytorch problem with Illegal instruction / Petr Kozelka
- PR#910: Implement support for k-LIME MOJO / Navdeep Gill
- PR#937: [CI] release libdaimojo on S3 / Qiang Kou (KK)
- PR#936: [cpp] separate Bert Tokenizer and SentencePiece Models / Qiang Kou (KK)
- PR#933: Minor cleanup in ReaderContext classes for TOML / Petr Kozelka
- PR#814: [cpp] upgrade pytorch to 1.3.0 / Qiang Kou (KK)
- PR#893: [java] integration testing for external mojos and datasets / Petr Kozelka
- PR#930: [cpp] add distilbert tokenizer / Qiang Kou (KK)
- PR#931: Removed dead tests / Petr Kozelka
- MojoTransformMomentTest: optimized imports to remove dead class dependencies (tiny) / Petr Kozelka
- PR#927: [py] including transformer vocab files in daimojo pkg / Qiang Kou (KK)
- PR#923: [cpp] delete tmp files generated in tensorflow transformer / Qiang Kou (KK)
- PR#924: [cpp] warning for custom op in pipeline / Qiang Kou (KK)
- PR#929, #928: [cpp] suffix of shared lib on OS X can be .so or .dylib / Qiang Kou (KK)
- PR#922: [cpp] update bert tokenizer following python pkg / Qiang Kou (KK)
- PR#919: eliminated error-prone separator configurability / Petr Kozelka
- PR#916: Remove dead code / Petr Kozelka
- PR#920, #853: [WIP][cpp] switch to json format for lightgbm model serialization / Qiang Kou (KK)
- PR#914: [java] Have stronger typing in MojoPipeline.loadFrom / Petr Kozelka
- PR#915: base score should be used as base margin if later is not available / Qiang Kou (KK)
- PR#912: [FIX] Load datetime from Protobuf / Michal Malohlava

## 2.4.0 - 2020-04-01

- PR#909: Explicitly check that CSV has same number of columns as its header row / Petr Kozelka
- PR#907: Exposed H2O-3 UUID / Edgar Orendain
- [py] add py/model2proto/bin/tree2txt / Qiang Kou
- PR#905: [DOC] update tree shap value notebook / Qiang Kou (KK)
- PR#901: Drop unnecessary SimpleCSV class / Petr Kozelka
- PR#795: Eliminated Javassist from MojoTransformIsNaBuilder / Petr Kozelka
- PR#904: Batched validation of predictions / Petr Kozelka
- PR#903: [java] tiny fix in deprecation of MojoPipelineReaderBackendFactory#createReaderBackend / Petr Kozelka
- PR#902, #900: [IDL] store transformed feature names / Qiang Kou (KK)
- PR#898: [ITEST] Include 1.8.5.1 migration state / Michal Malohlava
- PR#883: Mini batching / Petr Kozelka
- PR#897: [ITEST] Include 1.8.5.1 migration state / Michal Malohlava
- PR#891: [cpp] tree shap values of linear ensemble model / Qiang Kou (KK)
- [py] remove reference of mojo visualization code / Qiang Kou
- [py] visualization code has been moved into h2oai repo / Qiang Kou
- PR#895, #894: [cpp] not return transformed dataframe / Qiang Kou (KK)
- [R] fix roxygen2 upgrade / Qiang Kou
- PR#892: [cpp] more append methods for MojoFrame / Qiang Kou (KK)
- PR#890: [cpp] sub pipelines can have the same pred_contrib_names / Qiang Kou (KK)
- PR#889: [cpp] remove undocumented verbose option / Qiang Kou (KK)

## 2.3.1 - 2020-03-09

- PR#884: Improved parameters for CustomOp extensions / Qiang Kou (KK)
- PR#882: [cpp] using Google's double conversion library instead of Netlib's dtoa / Qiang Kou (KK)
- PR#881, #833: [cpp] use dtoa from python to convert floating point num to string / Qiang Kou (KK)
- PR#810: [Java] Addition of tensorflow transformer to Java runtime / nkalonia1
- notebook for treeshap value demo / Qiang Kou
- PR#857: Mojo must not be locked to single processing backend / Petr Kozelka
- PR#879: [py] check DAI version when pred_contribs=True / Qiang Kou (KK)
- PR#878: [py] return prediction, pred_contribs and transformed data when pred_contribs=True. / Qiang Kou (KK)
- PR#877, #876: [R] generate Rd files using roxygen2 / Qiang Kou (KK)

## 2.3.0 - 2020-02-24

- PR#869: Fix of h2o-3 runtime dependencies. / Michal Malohlava
- PR#875: [cpp] simplify tree serialization code / Qiang Kou (KK)
- PR#873, #870: Upgrade xgboost to 1.0.0 / Qiang Kou (KK)
- PR#872: [ITEST] Include 1.8.4.1 for migration testing. / Michal Malohlava
- PR#868: [cpp] TreeShap value test for lgbm model with multiple outputs / Qiang Kou (KK)
- PR#867: [cpp] TreeShap value test for lgbm model with single output / Qiang Kou (KK)
- PR#866: [cpp] test for TreeShap values from xgboost model with multiple outputs / Qiang Kou (KK)
- PR#865: [cpp] test for TreeShap values from xgboost dart model with multiple outputs / Qiang Kou (KK)
- PR#864: [cpp] test for TreeShap values from xgboost dart model / Qiang Kou (KK)
- PR#862: [cpp] add one test for TreeSHAP values / Qiang Kou (KK)
- PR#861: [cpp] TreeShap values from mojo file / Qiang Kou (KK)
- PR#860: [cpp] leaf nodes also need data count for SHAP values / Qiang Kou (KK)
- PR#859: [cpp] save base_score and expected value for tree model / Qiang Kou (KK)
- PR#858: [cpp] add pred_contribs parameter / Qiang Kou (KK)
- PR#856: [cpp] get transformed names from pipeline / Qiang Kou (KK)
- PR#854: [cpp] add TreeSHAP computation from xgboost / Qiang Kou (KK)
- PR#851: [BUILD] remove model2capnp / Qiang Kou (KK)
- Turns out my implementation of `%b` didn't handle the case for "Jan" properly. Fixed and added test (#852). close #832 / nkalonia1
- PR#849, #842: [cpp] switch to json for xgboost model serialization / Qiang Kou (KK)
- PR#846: [java] Cosmetic fix: avoid empty catch block, use the exception / Petr Kozelka
- [cpp] use -1 as NA value for std::tm / Qiang Kou (KK)
- PR#787: [JAVA] Implements a service to create custom transformation builders. / Michal Malohlava
- PR#797: [java] fixed timezone handling, by eliminated static initializer of the default timezone and using preferred TZ (UTC) explicitly / Petr Kozelka

## 2.2.0 - 2020-01-29

- PR#812: [ITEST] Include 1.8.1 state for integration testing. / Michal Malohlava
- PR#843: [cpp] not include lightgbm in model2proto / Qiang Kou (KK)
- PR#836: [java] Removed dead code / Petr Kozelka
- PR#840: [c++] PathElement and related functions / Qiang Kou (KK)
- PR#839: [c++] add has_treeshap method into transformer and pipeline / Qiang Kou (KK)
- PR#829: [IDL] DAI version in mojo file / Qiang Kou (KK)
- PR#837, #834: [cpp] make const binary transformer valid between different types / Qiang Kou (KK)

## 2.1.19 - 2020-01-16 [YANKED]

- PR#835: [cpp] remove usage of mktime, which will convert tm into local time / Qiang Kou (KK)
- [BUILD] Archive obfuscation mapping. / mmalohlava

## 2.1.18 - 2020-01-14

- PR#830, #828: [cpp] make tm_mday to be 1 if day field is missing / Qiang Kou (KK)
- [HOTFIX] :fire: Wrong grep expression in our Jenkinsfile / mmalohlava
- PR#826: [LICENSE] Add default license lookup into ~/.driverlessai/license.sig / Michal Malohlava
- PR#805: Stricter type assertion in Agg Transformer / nkalonia1
- PR#822: [cpp] do not use push_back for vector of flexible types / Qiang Kou (KK)

## 2.1.17 - 2020-01-04
## 2.1.16 - 2020-01-04

- PR#823: [cpp] fix memory leak in C API / Qiang Kou (KK)
- PR#820: [cpp] Return int64 NA when hashing NA values in FTRL / nkalonia1
- PR#821: [cpp] not capture input/output names in transformers / Qiang Kou (KK)

## 2.1.15 - 2020-01-02

- [test] update licence checking test / Qiang Kou

## 2.1.14 - 2019-12-31

- PR#816: [Java] Remove use of `Long.remainderUnsigned` java FTRL code / nkalonia1
- PR#819: [cpp] reduce memory usage for array of strings / Qiang Kou (KK)
- [cpp] add missing decref/incref for std::tm in flexible type / Qiang Kou
- PR#818: change tm to pointer / Qiang Kou (KK)
- [test] upgrade java runtime version in C++ unit test / Qiang Kou
- [py] add latency measurement into py/daimojo/bin/mojo_scoring_cli / Qiang Kou
- [BUILD] Change build settings / Michal Malohlava

## 2.1.13 - 2019-12-20

- PR#809: [java] matrix product between int columns and double/float matrix / Qiang Kou (KK)
- PR#815: [test] add test for the tree structure in protobuf against json from LightGBM / Qiang Kou (KK)
- remove usage of keras2tf / Qiang Kou
- remove unused keras2tf / Qiang Kou (KK)
- PR#801: [UBENCH] A simple framework to perform microbenchmarks using JMH. / Michal Malohlava
- PR#807: Mark type Bool as not numeric / Petr Kozelka
- fix s3 upload / Qiang Kou
- PR#799: [BUILD] update the python/R artifact releasing / Qiang Kou (KK)
- PR#808: [cpp][py] simplify CMake settings for different platforms / Qiang Kou (KK)
- PR#806: [cpp] update for Windows compiling / Qiang Kou (KK)
- PR#800: [cpp] provide wrapper for platform-dependent code  / Qiang Kou (KK)
- [cpp] switch to std::getenv / Qiang Kou (KK)
- PR#789: Implement support for H2O-3 MOJO / Tom Kraljevic
- PR#773: [cpp] enable matrix product between integer rows and float/double matrix / Qiang Kou (KK)
- PR#766, #764: [cpp][R] switch to C++ 98 for daimojo R pkg / Qiang Kou (KK)
- PR#798: [build] improve building speed / Qiang Kou (KK)
- [TEST] add tiny ftrl mojos for testing / Qiang Kou
- PR#765: Updates to Jenkinsfile to run Java MOJO tests on multiple Java versions / nkalonia1
- [vis] hotfix for binary ensemble model / Qiang Kou

## 2.1.12 - 2019-12-05

- PR#794, #793: [cpp] add linear model for XGB / Qiang Kou (KK)
- PR#792: [VIS] make sure output names of sub-pipeline are correct / Qiang Kou (KK)
- PR#790: [R] pkg name now has arch and OS names / Qiang Kou (KK)
- PR#788: [VIS] update the visualization code for CV structure / Qiang Kou (KK)
- [OSX][R] release source R pkg for OSX from Jenkins / Qiang Kou
- [OSX][R] release binary R pkg for OSX from Jenkins / Qiang Kou
- [R] no need to link against BLAS/gfortran / Qiang Kou
- PR#786: [OSX][R] release the source pkg from Jenkins / Qiang Kou (KK)
- PR#785: [R] release R pkg for OSX from Jenkins / Qiang Kou (KK)
- PR#784: [CI] include libtensorflow in OSX release / Qiang Kou (KK)
- PR#781: [CI] release OSX python scoring whl from jenkins / Qiang Kou (KK)
- PR#780: [vis] generate dot file for XGBoost/LightGBM tree model / Qiang Kou (KK)
- PR#779, #778: [vis] tweak the visualization code / Qiang Kou (KK)

## 2.1.11 - 2019-11-19

- PR#777: [Java] fix FTRL implementation / Qiang Kou (KK)

## 2.1.10 - 2019-11-15

- PR#751: [Java] Implementation of the FTRL transformer / nkalonia1
- PR#752: [BUILD] Remove generated proto code from the repository / Michal Malohlava
- PR#776: [ITEST] Fix generation of temporary directories / Michal Malohlava
- PR#771: [BUILD] not using e machines for testing, since they don't support AVX / Qiang Kou (KK)
- PR#775: [TEST] speed up generation of testing cases / Qiang Kou (KK)
- PR#774: [ITEST] Extend integration tests to support mojo output from multiple / Michal Malohlava
- [PY] not using GPU when converting keras models into TF format / Qiang Kou
- PR#747: [ITEST] include 1.8.0 for backward compatibility / anuvaidya
- PR#768: [TEST] Stress parallel MOJO load. / Michal Malohlava
- [PY] convert argparse results into float / Qiang Kou
- PR#770: [BUILD] switch to ppc64le/centos / Qiang Kou (KK)
- PR#769: [PY] add keras2tf script to save keras model in a separate process / Qiang Kou (KK)
- [BUILD][PUBLISH] Increase internal gradle socket timeouts to 5minutes. / mmalohlava

## 2.1.9 - 2019-11-08

- PR#745: Mojo pipeline is not serializable / Jakub Háva
- PR#761: Include liblightgbm in model2proto / Qiang Kou (KK)
- [BUILD] fix model2proto building / Qiang Kou
- PR#760, #759: Support "==" decision type in lightgbm model serialization / Qiang Kou (KK)
- PR#758: [TEST] Include 1.6.5 in mojo backward compatibility tests / achraf-mer
- PR#757: Mojo visualization using group information / Qiang Kou (KK)
- PR#755: [Java] Added a "strict" mode to MojoRowBuilder / nkalonia1
- PR#756: [IDL] add transformation group / Qiang Kou (KK)
- PR#753: [cpp] fixes for bert scoring / Qiang Kou (KK)
- PR#750: [cpp] change output of pytorch tokenizer to (input, segment, mask) / Qiang Kou (KK)
- PR#739, #738: [cpp] Pytorch scoring support and an MLP example / Qiang Kou (KK)
- [BUILD] Next devel version. / mmalohlava

## 2.1.8 - 2019-10-07

- PR#746: [cpp] Bert tokenizer / Qiang Kou (KK)
- PR#723: [Java] Bundle a renamed protobuf library into mojo2-runtime-impl jar / nkalonia1
- PR#743: [DOCS] Publish API Javadoc as part of jenkins / Michal Malohlava
- PR#737, #165: [cpp] flexible_type implementation cleanup / Qiang Kou (KK)

## 2.1.7 - 2019-10-02

- PR#744: Lock the usage of `classMap` to prevent multiple pipeline builds from attempting to concurrently create the same class. / nkalonia1
- PR#731: [cpp] using macro to remove duplicated code in mathematical transformers. / Qiang Kou (KK)
- [BUILD] Next dev cycle 2.1.7-SNAPSHOT / Qiang Kou

## 2.1.6 - 2019-09-22

- PR#710: [ITESTS] Include version 1.6.4, 1.7.0 and 1.7.1 in backward / Michal Malohlava
- PR#735: [cpp] ftrl testing case update / Qiang Kou (KK)
- PR#734: [cpp] Fix FTRL MOJO for OriginalTransformers by saving feature names / Qiang Kou (KK)
- [py] not using numpy in daimojo pkg / Qiang Kou (KK)
- PR#730: [TEST] add lightgbm random forest examples / Qiang Kou (KK)
- [BUILD] Next dev cycle 2.1.6-SNAPSHOT / Qiang Kou
- [BUILD] Hotfix :fire: - remove hardcoded IP of nexus repo. / mmalohlava

## 2.1.5 - 2019-09-12

- [BUILD] Next dev cycle 2.1.5-SNAPSHOT / Nishant Kalonia

## 2.1.4 - 2019-09-11

- PR#729: [Java] Compatibility with big-endian machines / nkalonia1
- PR#728: [py] update utility scripts / Qiang Kou (KK)
- [LGBM] add testing data for xentropy objective function / Qiang Kou
- PR#726: [cpp] fix log transformer. float -> double is allowed / Qiang Kou (KK)
- PR#725, #724: [cpp] allow integer types as input of log transformer / Qiang Kou (KK)
- PR#721: Revert "[cpp] freeze tensorflow graph for scoring (#719)" / Qiang Kou (KK)
- PR#720, #718: [JAVA] don't set 1.0/0.0 to be NA / nkalonia1
- PR#722: [cpp] add an option to show running time for each transformer.close # 669 / Qiang Kou (KK)
- PR#719: [cpp] freeze tensorflow graph for scoring / Qiang Kou (KK)
- PR#717: [cpp] Ftrl model scoring update / Qiang Kou (KK)
- PR#716: [cpp] add testing cases for FTRL regression/multinomial models. / Qiang Kou (KK)
- PR#714: [cpp] FTRL model scoring / Qiang Kou (KK)
- PR#715: Allow Int32 to Time64 conversions in AsTypeBuilder assertions / nkalonia1
- [CI] fix failures in python package building / Qiang Kou
- PR#712: [cpp] switch to murmur hashing used in datatable FTRL / Qiang Kou (KK)
- PR#711: [MOJO] update char-level tokenizer to accpet emoji / Qiang Kou (KK)
- PR#707: [cpp] refactor tf scoring code into a separate class / Qiang Kou (KK)
- PR#709: [cpp] refactor keras tokenizer into a separate class / Qiang Kou (KK)
- PR#706: [cpp] workaroud for h2oai/h2oai#9909 / Qiang Kou (KK)
- PR#704: [TEST] Add test to verify multiload of mojos / Michal Malohlava
- PR#703: Revert "[cpp] using frozen graphs in TF scoring (#702)" / Qiang Kou (KK)
- PR#702: [cpp] using frozen graphs in TF scoring / Qiang Kou (KK)
- PR#698, #697: [cpp] unicode handling in keras tokenizer / Qiang Kou (KK)
- PR#701: [tensorflow] fix libtensorflow loading in subpipeline / Qiang Kou (KK)
- PR#696: [Java] Throw a clearer error message when attempting to read a TensorFlow transformer / nkalonia1
- [BUILD] Next dev cycle 2.1.4-SNAPSHOT / mmalohlava
- PR#677: [cpp] remove unused functions from flexible_type / Qiang Kou (KK)

## 2.1.3 - 2019-08-11 [YANKED]

- [BUILD] Next dev cycle 2.1.4-SNAPSHOT / mmalohlava

## 2.1.3 - 2019-08-11

- PR#691: [HOTFIX] :ambulance: fix in memory loader / Michal Malohlava
- PR#688: [Java] Fix class casting issue when reading some TOML MOJOs / nkalonia1
- PR#692: [py] add sep option for csv parsing / Qiang Kou (KK)
- PR#689, #687: [R] include libtensorflow in R pkg / Qiang Kou (KK)
- [BUILD] Next dev cycle 2.1.3-SNAPSHOT / mmalohlava

## 2.1.2 - 2019-08-06

- PR#686: Fix method call typo / nkalonia1
- PR#685: [Java] Ensure that watermark pipeline file name matches backend / nkalonia1
- PR#684: [ITEST] Enable assertions / Michal Malohlava
- [java] bug fixes in some assert statements / Nishant Kalonia

## 2.1.1 - 2019-08-05

- PR#682: Introduce perpetual MOJO / Michal Malohlava
- [py] update setup.py for daimojo / Qiang Kou
- [py] add header=True for mojo_scoring_cli / Qiang Kou (KK)
- [py] add #! for mojo_scoring_cli / Qiang Kou
- PR#681, #680: [py] update packaging script for py package / Qiang Kou (KK)
- PR#679: [cpp] include libtensorflow in python scoring package / Qiang Kou (KK)
- :ambulance: extend deadline to run full suite of tests / mmalohlava
- [TEST] update testing for xgboost models with unused nodes / Qiang Kou (KK)

## 2.1.0 - 2019-07-22

- PR#674: [java] Allow undefined node types to appear in a tree's node list / nkalonia1
- PR#672: [TEST] add testing case for unused nodes in xgboost model / Qiang Kou (KK)
- PR#670: [DEPS] update lightgbm and xgboost following h2o4gpu / Qiang Kou (KK)
- [java] removed unused import statement / Nishant Kalonia
- PR#671: [java] Allow empty string labels when reading CSVs / nkalonia1
- PR#668: [cpp] input and output of sqrt transformer can be different types / Qiang Kou (KK)
- PR#667: [cpp] better error message for tensorflow transformer / Qiang Kou (KK)
- PR#666, #658: [R] use data.frame in base package / Qiang Kou (KK)
- PR#663: [cpp] add missing num_words in keras tokenizer / Qiang Kou (KK)
- PR#662: [cpp] upgrade tensorflow to 1.13.1 following DAI. / Qiang Kou (KK)
- PR#622: Refactor java TOML implementation to utilize new protobuf structure / nkalonia1
- PR#661: [cpp] add keras tokenizer transformer / Qiang Kou (KK)

## 2.0.2 - 2019-07-10

- PR#659: [cpp] delete treelite dependency / Qiang Kou (KK)
- PR#660: [test] add scripts to generate testing data for tensorflow NLP models / Qiang Kou (KK)
- PR#657: [cpp] flexible type and transformer clean up / Qiang Kou (KK)
- [java] fix bug where ConstBinaryOp transformer fails when creating DIVIDE instance with epsilon / Nishant Kalonia
- PR#656: [cpp] add copy and move constructor for flexible col type / Qiang Kou (KK)
- PR#655: [cpp] add flexible_col and simple tests for it / Qiang Kou (KK)
- PR#621: [cpp] add move ctor for flexible type / Qiang Kou (KK)
- PR#653: [cpp] fill na op cleanup / Qiang Kou (KK)
- PR#652: [cpp] median op cleanup / Qiang Kou (KK)
- PR#651: [cpp] impute na and moment cleanup / Qiang Kou (KK)
- PR#650: [cpp] using type info in clip transform / Qiang Kou (KK)
- [TEST] update license checking test / Qiang Kou
- PR#649: [cpp] output of normalize can be float32/float64 / Qiang Kou (KK)
- PR#648: [cpp] make output of CountVectorizer can be any numerical types / Qiang Kou (KK)
- PR#647: [cpp] softmax is only for float32/float64 / Qiang Kou (KK)
- PR#646: [cpp] scale op is only for numerical types and output can only be float32/float64 / Qiang Kou (KK)
- PR#645: [cpp] binary op only handles numerical types / Qiang Kou (KK)
- PR#644: [cpp] log is only for float32/float64 types / Qiang Kou (KK)
- PR#642: [cpp] use type info in const binary op / Qiang Kou (KK)
- PR#643: [cpp] expm1 can only be used for float32/float64 / Qiang Kou (KK)
- PR#640: [cpp] make as type transform 1 to 1 / Qiang Kou (KK)
- [cpp] remove snapshot from C++ lib version / Qiang Kou
- [TEST] remvoe R/python valgrind test. It may hang. / Qiang Kou
- PR#638: Update test_mojo_compatibility.py / Michal Malohlava

## 2.0.1 - 2019-06-26

- PR#635: [java] Implement protobuf reading of tree files / nkalonia1
- [java] Default runtime to read protobuf. Attempts to read toml if protobuf fails. (Closes #636) / Nishant Kalonia
- fix jenkins building / Qiang Kou (KK)
- PR#634: [MOJO] datetime can also be NA / Qiang Kou (KK)
- PR#632: [cpp] keys and values in map op can be empty / Qiang Kou (KK)

## 2.0.0 - 2019-06-24

- PR#630: [JAVA] use the new IDL version and patch version / Qiang Kou (KK)
- PR#629, #628: [JAVA] parse the UUID of mojo / Qiang Kou (KK)
- PR#627: [R] add installation instruction / Qiang Kou (KK)
- PR#626: [cpp] add PATCH_VERSION.txt and use it in C++ runtime version number / Qiang Kou (KK)
- PR#625: Update VERSION.txt / Michal Malohlava
- PR#606: [JAVA] allow double types to parsed as a float value / nkalonia1
- PR#624: [TEST] add testing data for missing types in lightgbm / Qiang Kou (KK)

## 0.13.26 - 2019-06-18

- PR#555: Switch from TOML to Protobuf for Java MOJO / nkalonia1
- PR#610: [JAVA] parse %y format following Linux strptime / Qiang Kou (KK)
- PR#620: [TEST] add C api test using two sigma dataset / Qiang Kou (KK)
- PR#618: [TEST] add two more testing mojo files from twosigma dataset / Qiang Kou (KK)
- PR#617: [cpp] fix unicode handling in map op / Qiang Kou (KK)
- [py/R] better error message for license checking / Qiang Kou (KK)
- PR#615: [cpp] fix the different behavior of ngram between C++ and Java runtime  / Qiang Kou (KK)
- [DOC] readme for upgrading xgboost and lightgbm / Qiang Kou
- PR#611, #613: [cpp] add weight drop of dart booster from xgboost / Qiang Kou (KK)
- [DOC] add license info for C++ dependencies / Qiang Kou
- PR#609, #608: [TEST] add xgboost dart test / Qiang Kou (KK)

## 0.13.25 - 2019-06-04

- PR#603: Obfuscate the jar pushed to public maven. / Ondrej Sery
- PR#602: Fix non-release build. / Ondrej Sery

## 0.13.24 - 2019-06-03

- PR#601: [cpp] misc cleanup / Qiang Kou (KK)
- PR#598: Add empty jars to bypass the Sonatype checks. / Ondrej Sery

## 0.13.23 - 2019-05-30

- Fix typo in publishing repositories. / Ondrej Sery

## 0.13.22 - 2019-05-30

- PR#596, #582: [TEST] more R pkg tests / Qiang Kou (KK)
- PR#595: [TEST] add twosigma test for c api / Qiang Kou (KK)
- PR#594: [py] update mem check in python scoring pkg / Qiang Kou (KK)
- PR#591: [R] add R unit test and memory check / Qiang Kou (KK)
- PR#592: [TEST] add memory check for python scoring pkg / Qiang Kou (KK)
- PR#576: Move project specific publishing info into their build files. / Ondrej Sery
- PR#575: Link the -impl pom to the commercial H2O.ai license. / Ondrej Sery
- PR#588: [cpp] int32/int64 to datetime with format string / Qiang Kou (KK)
- PR#589: [cpp] using unique ptr for Transformbase / Qiang Kou (KK)
- PR#587: [TEST] update the docker image for testing / Qiang Kou (KK)
- PR#550: Fix writing mojo to Toml String / Jakub Háva
- PR#585: [cpp] add uuid to c api / Qiang Kou (KK)
- PR#580, #565: [R] switch to C api in R pkg to avoid ABI problem / Qiang Kou (KK)
- PR#570: Make mojo impl .jar public. / Ondrej Sery

## 0.13.21 - 2019-05-23

- PR#583, #581: [TEST] add more unit test for C api / Qiang Kou (KK)
- PR#579: [cpp] cleanup usage of shared ptr in tree model / Qiang Kou (KK)
- PR#577: Driverless AI evaluation agreement for python/R scoring pkg and C scoring lib. / Qiang Kou (KK)
- PR#574: [cpp] expose more functions in C api / Qiang Kou (KK)
- PR#564: Hacky attempt to make the toml parser stream file / nkalonia1
- PR#569: [ITESTS] Include 1.6.0, 1.6.1.1, 1.6.2 into itests / Michal Malohlava
- [DOCKER] :ant: Fix references to `docker.h2o.ai` / mmalohlava
- updated registry to harbor.h2o.ai / Anmol Bal
- update xgboost and lightGBM with h2o4gpu / Qiang Kou
- PR#567: [DOC] update the daimojo python example / Qiang Kou (KK)
- PR#562: [cpp] add testing case for datetime format "%m/%d/%y" / Qiang Kou (KK)
- [BUILD] Next dev cycle / Qiang Kou

## 0.13.20 - 2019-05-05
## 0.19.6 - 2019-04-27

- PR#557: [R] fix R pkg building script / Qiang Kou (KK)
- PR#556: [R] publish the R package to S3 / Qiang Kou (KK)
- PR#548: [ITESTS] Enable migration state for 1.5.5 / Michal Malohlava
- PR#551, #549: [cpp] move unicode preprocessing into VocabularyCounter / Qiang Kou (KK)
- [BUILD] Next dev cycle / Qiang Kou

## 0.13.19 - 2019-04-14
## 0.19.5 - 2019-04-10

- [BUILD] Next dev cycle / Qiang Kou
- PR#552, #510: [java] add int32 to datetime transfomer  / Qiang Kou (KK)
- [py] dai_mojo -> daimojo / Qiang Kou

## 0.19.4 - 2019-04-05

- PR#545: [py] separate wheels for different architecture / Qiang Kou (KK)
- PR#541: [py] bundle so files for x86 and power in the same wheel / Qiang Kou (KK)
- PR#540: [R] package update and add README / Qiang Kou (KK)
- PR#539, #536: [cpp] do not threw exception when no license found / Qiang Kou (KK)
- PR#535: [py] move visualization code into model2proto pkg / Qiang Kou (KK)
- PR#532, #426: [R] mojo scoring package in R / Qiang Kou (KK)
- PR#525: [cpp] use mutable get in passing data from python to C++ / Qiang Kou (KK)
- PR#524, #404: [cpp] tree model refactor / Qiang Kou (KK)
- PR#523: [cpp] put col type info into each transformer / Qiang Kou (KK)
- PR#521: [cpp] clean up kmeans transform  / Qiang Kou (KK)
- PR#520, #519: [cpp] cleanup normalization transform  / Qiang Kou (KK)
- PR#517, #516: [cpp] clean up matrix product transformer / Qiang Kou (KK)
- PR#499: [ITESTS] Enable backward compatibility testing for DAI 1.5.2, 1.5.3, and 1.5.4 / Michal Malohlava
- PR#514, #513: [cpp] license checking in C++ / Qiang Kou (KK)
- add a simple visualization script protobuf-based mojo / Qiang Kou
- PR#505, #202: [cpp] binary tree implementation / Qiang Kou (KK)
- PR#509: [TEST] add one more testing case / Qiang Kou (KK)

## 2.19.3 - 2019-03-18

- PR#504: [cpp] tree model implementation cleanup / Qiang Kou (KK)

## 2.19.2 - 2019-03-14

- PR#497: [cpp] fix glm from xgboost / Qiang Kou (KK)
- PR#496: [cpp] fix the comparison between NAs / Qiang Kou (KK)
- PR#495: [py] add -DNDEBUG in whl building. / Qiang Kou (KK)
- PR#493, #486: [cpp] switch to static linking / Qiang Kou (KK)
- PR#491, #490: [cpp] build libtensorflow without AVX2 to support old CPUs / Qiang Kou (KK)
- PR#489, #487: [cpp] C api for mojo runtime / Qiang Kou (KK)
- [BUILD] Next dev cycle / mmalohlava
- PR#484: [cpp] preliminary tf support / Qiang Kou (KK)
- [cpp] remove ShutdownProtobufLibrary in testing code.  / Qiang Kou (KK)
- PR#481: [cpp] switch to protobuf3.6.0 to be consistent with tensorflow 1.11.0 / Qiang Kou (KK)
- PR#477: [cpp] add proto definition for tensorflow op / Qiang Kou (KK)
- PR#469: [cpp] clean up tree model implementation. / Qiang Kou (KK)

## 0.13.18 - 2019-02-14

- PR#474: [cpp] update moment transformer test with java / Qiang Kou (KK)
- [ITESTS] Fix improper collection of dataset location / mmalohlava

## 0.13.17 - 2019-02-13

- PR#473: [FIX] Fixes wrong array size in MomentumTransformer / Michal Malohlava
- PR#472: [cpp] remove linear model implementation / Qiang Kou (KK)
- PR#470: [cpp] remove float tree for lightGBM / Qiang Kou (KK)
- PR#468: [cpp] xgb/lightgbm code cleanup / Qiang Kou (KK)
- PR#465: [TEST] generate java runtime result during unit test / Qiang Kou (KK)

## 0.13.16 - 2019-02-07

- PR#464: [cpp] make first breakpoint left-inclusive / Qiang Kou (KK)

