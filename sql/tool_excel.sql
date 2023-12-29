CREATE TABLE IF NOT EXISTS `tool_excel` (
  `excel_id` longtext COLLATE utf8_unicode_ci NOT NULL,
  `excel_name` text COLLATE utf8_unicode_ci NOT NULL,
  `excel_data` longtext COLLATE utf8_unicode_ci,
  `user_id` bigint(20) NOT NULL,
  `create_time` datetime DEFAULT NULL,
  `update_time` datetime DEFAULT NULL,
  PRIMARY KEY (`excel_id`(100)) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci COMMENT='在线文档';