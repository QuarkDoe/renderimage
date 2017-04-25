#include "renderimage.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    RenderImage w;
    w.show();

    return app.exec();
}

