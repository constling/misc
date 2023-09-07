#include "skyutil.h"
#include <QCoreApplication>
#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonArray>


QString SKYUtil::applicationDir()
{
    return QCoreApplication::applicationDirPath();
}

bool SKYUtil::isFileExist(const QString &path) {
    QDir dir(path);
    QString usePath = dir.absoluteFilePath(path);
    QFile file(usePath);
    return file.exists();
}

bool SKYUtil::isDirExist(QString fullPath)
{
     QDir dir(fullPath);
     if(dir.exists())
     {
       return true;
     }
    return false;
}

QString SKYUtil::readFileByText(const QString &path)
{
    QDir dir(path);
    QString usePath = dir.absoluteFilePath(path);
    QFile file(usePath);
    if (!file.open(QFile::ReadOnly)) {
        return QString();
    } else {
        QTextStream in(&file);
        return in.readAll();
    }
}

bool SKYUtil::getStratParamFromFile(const QString &filePath, tagStartParam &result)
{
    QString text = readFileByText(filePath);
    if (text.isEmpty()) {
        return false;
    }

    QJsonDocument jsonDoc = QJsonDocument::fromJson(text.toUtf8());
    if (!jsonDoc.isObject()) return false;

    QJsonObject jsonObj = jsonDoc.object();

    result.version = jsonObj["version"].toString();
    result.relativePath = jsonObj["relative"].toString();
    if (jsonObj["param"].isArray()) {
        QJsonArray jsonArray = jsonObj["param"].toArray();
        for (int index = 0; index < jsonArray.size(); ++index) {
            result.params.append(jsonArray.at(index).toString());
        }
    }
    return true;
}
