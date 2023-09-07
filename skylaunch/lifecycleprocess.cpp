#include "lifecycleprocess.h"
#include <QApplication>
#include <QCoreApplication>


LifecycleProcess::LifecycleProcess(QObject *parent)
    : QObject{parent}
{

}


void LifecycleProcess::start(int timelong)
{
    m_timer = startTimer(timelong);
}

void LifecycleProcess::timerEvent(QTimerEvent *e)
{
    ((QApplication *)QCoreApplication::instance())->exit();
}
