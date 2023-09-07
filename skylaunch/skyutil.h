#ifndef SKYUTIL_H
#define SKYUTIL_H
#include <QString>
#include "skydefine.h"

class SKYUtil
{
public:
    // 应用目录
    static QString applicationDir();

    // 文件是否存在
    static bool isFileExist(const QString &path);

    // 目录是否存在
    static bool isDirExist(QString fullPath);

    // 读取文件信息
    static QString readFileByText(const QString &path);

    // 获取启动参数
    static bool getStratParamFromFile(const QString &filePath, tagStartParam &result);
};

#endif // SKYUTIL_H
