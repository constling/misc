QT       += core

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++17

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    lifecycleprocess.cpp \
    main.cpp \
    skyutil.cpp

HEADERS += \
    lifecycleprocess.h \
    skydefine.h \
    skyutil.h

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target


win32 : {
    QMAKE_CXXFLAGS_EXCEPTIONS_ON = /EHa
    QMAKE_CXXFLAGS_STL_ON = /EHa

    CONFIG(release, debug|release) {
        QMAKE_LFLAGS += /MANIFESTUAC:\"level=\'requireAdministrator\' uiAccess=\'false\'\"
    }
}

RC_FILE += logo.rc

DISTFILES += \
    logo.rc

# 配置输出目录
CONFIG(release,debug|release){
    DESTDIR = ./LaunchRelease
}

CONFIG(debug,debug|release){
    DESTDIR = ./LaunchDebug
}
