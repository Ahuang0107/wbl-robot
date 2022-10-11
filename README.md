# wbl-robot

# 使用步骤

- 在27017端口开启mongodb服务
- mongodb新建一个叫wbl_robot的database
- wbl_robot中新建一个叫questions的collection
- 解压bank中的questions.zip
- mongodb导入解压得到的questions.json
- 复制.env.sample文件并重命名为.env
- 打开测试页面控制台，找到一条请求，在Request URL中找到quiz后的id，Request Headers中找到cookie和owasp_csrftoken，复制到.env中
- 双击wbl-robot.exe开始答题

### 使用后记得再将mongodb的questions数据导出，压缩成zip包覆盖原来的questions.zip，然后上传

