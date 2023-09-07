
#include <QApplication>
#include <QDir>
#include <QMessageBox>
#include <QProcess>
#include "skyutil.h"
#include "lifecycleprocess.h"



int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    QString iniPath = SKYUtil::applicationDir() + QDir::separator() + "skygame.ini";

    tagStartParam startParam;
    if (false == SKYUtil::getStratParamFromFile(iniPath, startParam)) {
        QMessageBox box(QMessageBox::NoIcon,"","版本配置读取失败，确认版本配置");
        box.exec();
        return true;
    }

    QString launchExePath = SKYUtil::applicationDir() + QDir::separator() + startParam.version + QDir::separator() + "skygame.exe";
    if (!startParam.relativePath.isEmpty()) {
        launchExePath = startParam.relativePath + QDir::separator() + "skygame.exe";
    }

    if (false == SKYUtil::isFileExist(launchExePath)) {
        QMessageBox box(QMessageBox::NoIcon,"","启动文件不存在，确认版本配置");
        box.exec();
        return true;
    }

    QProcess process;
    if (false == process.startDetached(launchExePath, startParam.params)) {
        QMessageBox box(QMessageBox::NoIcon,"","启动进程失败，确认安装是否异常");
        box.exec();
        return true;
    }

    process.waitForFinished();
    static LifecycleProcess lifecycle;
    lifecycle.start(5000);

    return a.exec();
}
