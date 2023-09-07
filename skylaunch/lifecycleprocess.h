#ifndef LIFECYCLEPROCESS_H
#define LIFECYCLEPROCESS_H

#include <QObject>

class LifecycleProcess : public QObject
{
    Q_OBJECT
public:
    explicit LifecycleProcess(QObject *parent = nullptr);

    void start(int timelong);

protected:
    void timerEvent(QTimerEvent *e);

private:
    int m_timer = 0;
};

#endif // LIFECYCLEPROCESS_H
