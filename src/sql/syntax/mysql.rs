//! MySQL 语法定义

use super::common::{SqlCompletion, SqlSyntax};

/// MySQL 语法
pub struct MySqlSyntax;

impl SqlSyntax for MySqlSyntax {
    fn keywords(&self) -> Vec<SqlCompletion> {
        vec![
            // DML
            SqlCompletion::keyword("SELECT", "查询数据"),
            SqlCompletion::keyword("INSERT", "插入数据"),
            SqlCompletion::keyword("UPDATE", "更新数据"),
            SqlCompletion::keyword("DELETE", "删除数据"),
            SqlCompletion::keyword("REPLACE", "替换数据"),
            
            // 子句
            SqlCompletion::keyword("FROM", "数据来源"),
            SqlCompletion::keyword("WHERE", "条件过滤"),
            SqlCompletion::keyword("AND", "与条件"),
            SqlCompletion::keyword("OR", "或条件"),
            SqlCompletion::keyword("NOT", "非条件"),
            SqlCompletion::keyword("IN", "包含"),
            SqlCompletion::keyword("LIKE", "模糊匹配"),
            SqlCompletion::keyword("BETWEEN", "范围"),
            SqlCompletion::keyword("IS NULL", "空值判断"),
            SqlCompletion::keyword("IS NOT NULL", "非空判断"),
            SqlCompletion::keyword("EXISTS", "存在判断"),
            SqlCompletion::keyword("ORDER BY", "排序"),
            SqlCompletion::keyword("GROUP BY", "分组"),
            SqlCompletion::keyword("HAVING", "分组过滤"),
            SqlCompletion::keyword("LIMIT", "限制行数"),
            SqlCompletion::keyword("OFFSET", "偏移量"),
            SqlCompletion::keyword("DISTINCT", "去重"),
            SqlCompletion::keyword("AS", "别名"),
            SqlCompletion::keyword("ON", "连接条件"),
            SqlCompletion::keyword("USING", "使用列连接"),
            
            // JOIN
            SqlCompletion::keyword("JOIN", "连接表"),
            SqlCompletion::keyword("INNER JOIN", "内连接"),
            SqlCompletion::keyword("LEFT JOIN", "左连接"),
            SqlCompletion::keyword("LEFT OUTER JOIN", "左外连接"),
            SqlCompletion::keyword("RIGHT JOIN", "右连接"),
            SqlCompletion::keyword("RIGHT OUTER JOIN", "右外连接"),
            SqlCompletion::keyword("CROSS JOIN", "交叉连接"),
            SqlCompletion::keyword("NATURAL JOIN", "自然连接"),
            
            // UNION
            SqlCompletion::keyword("UNION", "合并结果"),
            SqlCompletion::keyword("UNION ALL", "合并所有结果"),
            
            // DDL
            SqlCompletion::keyword("CREATE", "创建对象"),
            SqlCompletion::keyword("CREATE TABLE", "创建表"),
            SqlCompletion::keyword("CREATE DATABASE", "创建数据库"),
            SqlCompletion::keyword("CREATE INDEX", "创建索引"),
            SqlCompletion::keyword("CREATE VIEW", "创建视图"),
            SqlCompletion::keyword("CREATE PROCEDURE", "创建存储过程"),
            SqlCompletion::keyword("CREATE FUNCTION", "创建函数"),
            SqlCompletion::keyword("CREATE TRIGGER", "创建触发器"),
            SqlCompletion::keyword("CREATE USER", "创建用户"),
            SqlCompletion::keyword("ALTER", "修改对象"),
            SqlCompletion::keyword("ALTER TABLE", "修改表"),
            SqlCompletion::keyword("ALTER DATABASE", "修改数据库"),
            SqlCompletion::keyword("DROP", "删除对象"),
            SqlCompletion::keyword("DROP TABLE", "删除表"),
            SqlCompletion::keyword("DROP DATABASE", "删除数据库"),
            SqlCompletion::keyword("DROP INDEX", "删除索引"),
            SqlCompletion::keyword("TRUNCATE", "清空表"),
            SqlCompletion::keyword("RENAME", "重命名"),
            
            // 表结构
            SqlCompletion::keyword("ADD", "添加列"),
            SqlCompletion::keyword("MODIFY", "修改列"),
            SqlCompletion::keyword("CHANGE", "改变列"),
            SqlCompletion::keyword("DROP COLUMN", "删除列"),
            SqlCompletion::keyword("PRIMARY KEY", "主键"),
            SqlCompletion::keyword("FOREIGN KEY", "外键"),
            SqlCompletion::keyword("UNIQUE", "唯一约束"),
            SqlCompletion::keyword("INDEX", "索引"),
            SqlCompletion::keyword("DEFAULT", "默认值"),
            SqlCompletion::keyword("NOT NULL", "非空约束"),
            SqlCompletion::keyword("AUTO_INCREMENT", "自增"),
            SqlCompletion::keyword("COMMENT", "注释"),
            SqlCompletion::keyword("ENGINE", "存储引擎"),
            SqlCompletion::keyword("CHARSET", "字符集"),
            SqlCompletion::keyword("COLLATE", "排序规则"),
            SqlCompletion::keyword("REFERENCES", "引用"),
            SqlCompletion::keyword("ON DELETE", "删除时"),
            SqlCompletion::keyword("ON UPDATE", "更新时"),
            SqlCompletion::keyword("CASCADE", "级联"),
            SqlCompletion::keyword("SET NULL", "置空"),
            SqlCompletion::keyword("RESTRICT", "限制"),
            
            // DCL
            SqlCompletion::keyword("GRANT", "授权"),
            SqlCompletion::keyword("REVOKE", "撤销权限"),
            SqlCompletion::keyword("FLUSH PRIVILEGES", "刷新权限"),
            
            // TCL
            SqlCompletion::keyword("BEGIN", "开始事务"),
            SqlCompletion::keyword("START TRANSACTION", "开始事务"),
            SqlCompletion::keyword("COMMIT", "提交事务"),
            SqlCompletion::keyword("ROLLBACK", "回滚事务"),
            SqlCompletion::keyword("SAVEPOINT", "保存点"),
            SqlCompletion::keyword("SET AUTOCOMMIT", "自动提交"),
            
            // 其他
            SqlCompletion::keyword("SHOW", "显示信息"),
            SqlCompletion::keyword("SHOW TABLES", "显示表"),
            SqlCompletion::keyword("SHOW DATABASES", "显示数据库"),
            SqlCompletion::keyword("SHOW COLUMNS", "显示列"),
            SqlCompletion::keyword("SHOW INDEX", "显示索引"),
            SqlCompletion::keyword("SHOW CREATE TABLE", "显示建表语句"),
            SqlCompletion::keyword("SHOW PROCESSLIST", "显示进程"),
            SqlCompletion::keyword("SHOW STATUS", "显示状态"),
            SqlCompletion::keyword("SHOW VARIABLES", "显示变量"),
            SqlCompletion::keyword("DESCRIBE", "描述表结构"),
            SqlCompletion::keyword("DESC", "描述表结构"),
            SqlCompletion::keyword("EXPLAIN", "执行计划"),
            SqlCompletion::keyword("USE", "切换数据库"),
            SqlCompletion::keyword("SET", "设置变量"),
            SqlCompletion::keyword("INTO", "插入目标"),
            SqlCompletion::keyword("VALUES", "值列表"),
            SqlCompletion::keyword("VALUE", "值"),
            SqlCompletion::keyword("SET", "设置"),
            SqlCompletion::keyword("CASE", "条件表达式"),
            SqlCompletion::keyword("WHEN", "当"),
            SqlCompletion::keyword("THEN", "则"),
            SqlCompletion::keyword("ELSE", "否则"),
            SqlCompletion::keyword("END", "结束"),
            SqlCompletion::keyword("IF", "如果"),
            SqlCompletion::keyword("IFNULL", "空值替换"),
            SqlCompletion::keyword("NULLIF", "相等返回空"),
            SqlCompletion::keyword("COALESCE", "第一个非空"),
            SqlCompletion::keyword("ASC", "升序"),
            SqlCompletion::keyword("DESC", "降序"),
            SqlCompletion::keyword("ALL", "所有"),
            SqlCompletion::keyword("ANY", "任一"),
            SqlCompletion::keyword("SOME", "某些"),
            SqlCompletion::keyword("TRUE", "真"),
            SqlCompletion::keyword("FALSE", "假"),
            SqlCompletion::keyword("NULL", "空值"),
            SqlCompletion::keyword("LOCK TABLES", "锁表"),
            SqlCompletion::keyword("UNLOCK TABLES", "解锁表"),
        ]
    }
    
    fn functions(&self) -> Vec<SqlCompletion> {
        vec![
            // 聚合函数
            SqlCompletion::function("COUNT()", "计数"),
            SqlCompletion::function("SUM()", "求和"),
            SqlCompletion::function("AVG()", "平均值"),
            SqlCompletion::function("MAX()", "最大值"),
            SqlCompletion::function("MIN()", "最小值"),
            SqlCompletion::function("GROUP_CONCAT()", "分组连接"),
            
            // 字符串函数
            SqlCompletion::function("CONCAT()", "连接字符串"),
            SqlCompletion::function("CONCAT_WS()", "带分隔符连接"),
            SqlCompletion::function("SUBSTRING()", "子字符串"),
            SqlCompletion::function("SUBSTR()", "子字符串"),
            SqlCompletion::function("LEFT()", "左截取"),
            SqlCompletion::function("RIGHT()", "右截取"),
            SqlCompletion::function("LENGTH()", "字节长度"),
            SqlCompletion::function("CHAR_LENGTH()", "字符长度"),
            SqlCompletion::function("UPPER()", "转大写"),
            SqlCompletion::function("LOWER()", "转小写"),
            SqlCompletion::function("TRIM()", "去空格"),
            SqlCompletion::function("LTRIM()", "去左空格"),
            SqlCompletion::function("RTRIM()", "去右空格"),
            SqlCompletion::function("REPLACE()", "替换"),
            SqlCompletion::function("REVERSE()", "反转"),
            SqlCompletion::function("LPAD()", "左填充"),
            SqlCompletion::function("RPAD()", "右填充"),
            SqlCompletion::function("INSTR()", "查找位置"),
            SqlCompletion::function("LOCATE()", "查找位置"),
            SqlCompletion::function("POSITION()", "查找位置"),
            SqlCompletion::function("FORMAT()", "格式化数字"),
            SqlCompletion::function("SPACE()", "生成空格"),
            SqlCompletion::function("REPEAT()", "重复字符串"),
            SqlCompletion::function("ASCII()", "ASCII 码"),
            SqlCompletion::function("CHAR()", "ASCII 字符"),
            SqlCompletion::function("ORD()", "字符编码"),
            
            // 数学函数
            SqlCompletion::function("ABS()", "绝对值"),
            SqlCompletion::function("CEIL()", "向上取整"),
            SqlCompletion::function("CEILING()", "向上取整"),
            SqlCompletion::function("FLOOR()", "向下取整"),
            SqlCompletion::function("ROUND()", "四舍五入"),
            SqlCompletion::function("TRUNCATE()", "截断"),
            SqlCompletion::function("MOD()", "取模"),
            SqlCompletion::function("POW()", "幂"),
            SqlCompletion::function("POWER()", "幂"),
            SqlCompletion::function("SQRT()", "平方根"),
            SqlCompletion::function("EXP()", "指数"),
            SqlCompletion::function("LOG()", "对数"),
            SqlCompletion::function("LOG10()", "常用对数"),
            SqlCompletion::function("LOG2()", "二进制对数"),
            SqlCompletion::function("RAND()", "随机数"),
            SqlCompletion::function("SIGN()", "符号"),
            SqlCompletion::function("PI()", "圆周率"),
            SqlCompletion::function("SIN()", "正弦"),
            SqlCompletion::function("COS()", "余弦"),
            SqlCompletion::function("TAN()", "正切"),
            SqlCompletion::function("ASIN()", "反正弦"),
            SqlCompletion::function("ACOS()", "反余弦"),
            SqlCompletion::function("ATAN()", "反正切"),
            SqlCompletion::function("COT()", "余切"),
            SqlCompletion::function("DEGREES()", "弧度转角度"),
            SqlCompletion::function("RADIANS()", "角度转弧度"),
            
            // 日期函数
            SqlCompletion::function("NOW()", "当前时间"),
            SqlCompletion::function("CURDATE()", "当前日期"),
            SqlCompletion::function("CURTIME()", "当前时间"),
            SqlCompletion::function("CURRENT_DATE()", "当前日期"),
            SqlCompletion::function("CURRENT_TIME()", "当前时间"),
            SqlCompletion::function("CURRENT_TIMESTAMP()", "当前时间戳"),
            SqlCompletion::function("DATE()", "提取日期"),
            SqlCompletion::function("TIME()", "提取时间"),
            SqlCompletion::function("YEAR()", "提取年"),
            SqlCompletion::function("MONTH()", "提取月"),
            SqlCompletion::function("DAY()", "提取日"),
            SqlCompletion::function("DAYOFWEEK()", "星期几"),
            SqlCompletion::function("DAYOFMONTH()", "月中第几天"),
            SqlCompletion::function("DAYOFYEAR()", "年中第几天"),
            SqlCompletion::function("WEEKDAY()", "工作日"),
            SqlCompletion::function("WEEK()", "第几周"),
            SqlCompletion::function("HOUR()", "提取时"),
            SqlCompletion::function("MINUTE()", "提取分"),
            SqlCompletion::function("SECOND()", "提取秒"),
            SqlCompletion::function("MICROSECOND()", "提取微秒"),
            SqlCompletion::function("DATE_ADD()", "日期加"),
            SqlCompletion::function("DATE_SUB()", "日期减"),
            SqlCompletion::function("ADDDATE()", "日期加"),
            SqlCompletion::function("SUBDATE()", "日期减"),
            SqlCompletion::function("DATEDIFF()", "日期差"),
            SqlCompletion::function("TIMEDIFF()", "时间差"),
            SqlCompletion::function("TIMESTAMPDIFF()", "时间戳差"),
            SqlCompletion::function("DATE_FORMAT()", "日期格式化"),
            SqlCompletion::function("TIME_FORMAT()", "时间格式化"),
            SqlCompletion::function("STR_TO_DATE()", "字符串转日期"),
            SqlCompletion::function("UNIX_TIMESTAMP()", "Unix 时间戳"),
            SqlCompletion::function("FROM_UNIXTIME()", "时间戳转时间"),
            SqlCompletion::function("LAST_DAY()", "月末日期"),
            SqlCompletion::function("QUARTER()", "季度"),
            
            // 条件函数
            SqlCompletion::function("IF()", "条件判断"),
            SqlCompletion::function("IFNULL()", "空值替换"),
            SqlCompletion::function("NULLIF()", "相等返回空"),
            SqlCompletion::function("COALESCE()", "第一个非空"),
            SqlCompletion::function("GREATEST()", "最大值"),
            SqlCompletion::function("LEAST()", "最小值"),
            SqlCompletion::function("ISNULL()", "是否为空"),
            
            // 类型转换
            SqlCompletion::function("CAST()", "类型转换"),
            SqlCompletion::function("CONVERT()", "类型转换"),
            SqlCompletion::function("BINARY()", "转二进制"),
            
            // JSON 函数 (MySQL 5.7+)
            SqlCompletion::function("JSON_EXTRACT()", "提取 JSON"),
            SqlCompletion::function("JSON_UNQUOTE()", "去引号"),
            SqlCompletion::function("JSON_OBJECT()", "创建 JSON 对象"),
            SqlCompletion::function("JSON_ARRAY()", "创建 JSON 数组"),
            SqlCompletion::function("JSON_CONTAINS()", "JSON 包含"),
            SqlCompletion::function("JSON_KEYS()", "JSON 键"),
            SqlCompletion::function("JSON_LENGTH()", "JSON 长度"),
            SqlCompletion::function("JSON_TYPE()", "JSON 类型"),
            SqlCompletion::function("JSON_VALID()", "JSON 验证"),
            SqlCompletion::function("JSON_SET()", "设置 JSON"),
            SqlCompletion::function("JSON_INSERT()", "插入 JSON"),
            SqlCompletion::function("JSON_REPLACE()", "替换 JSON"),
            SqlCompletion::function("JSON_REMOVE()", "删除 JSON"),
            
            // 系统函数
            SqlCompletion::function("DATABASE()", "当前数据库"),
            SqlCompletion::function("SCHEMA()", "当前模式"),
            SqlCompletion::function("USER()", "当前用户"),
            SqlCompletion::function("CURRENT_USER()", "当前用户"),
            SqlCompletion::function("SESSION_USER()", "会话用户"),
            SqlCompletion::function("SYSTEM_USER()", "系统用户"),
            SqlCompletion::function("VERSION()", "MySQL 版本"),
            SqlCompletion::function("CONNECTION_ID()", "连接 ID"),
            SqlCompletion::function("LAST_INSERT_ID()", "最后插入 ID"),
            SqlCompletion::function("ROW_COUNT()", "影响行数"),
            SqlCompletion::function("FOUND_ROWS()", "找到行数"),
            SqlCompletion::function("UUID()", "生成 UUID"),
            SqlCompletion::function("UUID_SHORT()", "短 UUID"),
            SqlCompletion::function("MD5()", "MD5 哈希"),
            SqlCompletion::function("SHA1()", "SHA1 哈希"),
            SqlCompletion::function("SHA2()", "SHA2 哈希"),
            SqlCompletion::function("PASSWORD()", "密码哈希"),
            SqlCompletion::function("ENCRYPT()", "加密"),
            SqlCompletion::function("BENCHMARK()", "性能测试"),
            SqlCompletion::function("SLEEP()", "暂停"),
        ]
    }
    
    fn data_types(&self) -> Vec<SqlCompletion> {
        vec![
            // 整数
            SqlCompletion::data_type("TINYINT", "1 字节整数"),
            SqlCompletion::data_type("SMALLINT", "2 字节整数"),
            SqlCompletion::data_type("MEDIUMINT", "3 字节整数"),
            SqlCompletion::data_type("INT", "4 字节整数"),
            SqlCompletion::data_type("INTEGER", "4 字节整数"),
            SqlCompletion::data_type("BIGINT", "8 字节整数"),
            
            // 浮点
            SqlCompletion::data_type("FLOAT", "单精度浮点"),
            SqlCompletion::data_type("DOUBLE", "双精度浮点"),
            SqlCompletion::data_type("DECIMAL", "定点数"),
            SqlCompletion::data_type("NUMERIC", "定点数"),
            
            // 字符串
            SqlCompletion::data_type("CHAR", "定长字符串"),
            SqlCompletion::data_type("VARCHAR", "变长字符串"),
            SqlCompletion::data_type("TINYTEXT", "微型文本"),
            SqlCompletion::data_type("TEXT", "文本"),
            SqlCompletion::data_type("MEDIUMTEXT", "中型文本"),
            SqlCompletion::data_type("LONGTEXT", "长文本"),
            
            // 二进制
            SqlCompletion::data_type("BINARY", "定长二进制"),
            SqlCompletion::data_type("VARBINARY", "变长二进制"),
            SqlCompletion::data_type("TINYBLOB", "微型二进制"),
            SqlCompletion::data_type("BLOB", "二进制对象"),
            SqlCompletion::data_type("MEDIUMBLOB", "中型二进制"),
            SqlCompletion::data_type("LONGBLOB", "长二进制"),
            
            // 日期时间
            SqlCompletion::data_type("DATE", "日期"),
            SqlCompletion::data_type("TIME", "时间"),
            SqlCompletion::data_type("DATETIME", "日期时间"),
            SqlCompletion::data_type("TIMESTAMP", "时间戳"),
            SqlCompletion::data_type("YEAR", "年份"),
            
            // 其他
            SqlCompletion::data_type("BOOLEAN", "布尔值"),
            SqlCompletion::data_type("BOOL", "布尔值"),
            SqlCompletion::data_type("ENUM", "枚举"),
            SqlCompletion::data_type("SET", "集合"),
            SqlCompletion::data_type("JSON", "JSON 类型"),
            SqlCompletion::data_type("GEOMETRY", "几何类型"),
            SqlCompletion::data_type("POINT", "点"),
            SqlCompletion::data_type("LINESTRING", "线"),
            SqlCompletion::data_type("POLYGON", "多边形"),
        ]
    }
    
    fn operators(&self) -> Vec<SqlCompletion> {
        vec![
            SqlCompletion::keyword("=", "等于"),
            SqlCompletion::keyword("<>", "不等于"),
            SqlCompletion::keyword("!=", "不等于"),
            SqlCompletion::keyword(">", "大于"),
            SqlCompletion::keyword("<", "小于"),
            SqlCompletion::keyword(">=", "大于等于"),
            SqlCompletion::keyword("<=", "小于等于"),
            SqlCompletion::keyword("<=>", "安全等于（NULL 安全）"),
            SqlCompletion::keyword("REGEXP", "正则匹配"),
            SqlCompletion::keyword("RLIKE", "正则匹配"),
            SqlCompletion::keyword("SOUNDS LIKE", "发音相似"),
        ]
    }
    
    fn snippets(&self) -> Vec<SqlCompletion> {
        vec![
            SqlCompletion {
                text: "SELECT * FROM ".to_string(),
                description: "查询所有列".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "SELECT COUNT(*) FROM ".to_string(),
                description: "统计行数".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "INSERT INTO  () VALUES ()".to_string(),
                description: "插入数据".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "UPDATE  SET  WHERE ".to_string(),
                description: "更新数据".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "DELETE FROM  WHERE ".to_string(),
                description: "删除数据".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "CREATE TABLE  (\n  id INT AUTO_INCREMENT PRIMARY KEY,\n  \n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4".to_string(),
                description: "创建表模板".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "ALTER TABLE  ADD COLUMN ".to_string(),
                description: "添加列".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
            SqlCompletion {
                text: "CREATE INDEX  ON  ()".to_string(),
                description: "创建索引".to_string(),
                kind: super::common::SqlCompletionKind::Snippet,
            },
        ]
    }
}

