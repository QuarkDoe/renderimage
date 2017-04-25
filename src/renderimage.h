#ifndef RENDERIMAGE_H
#define RENDERIMAGE_H

#include <QMainWindow>

#include "datagenerator.h"
#include "workerthread.h"
#include "utils/container.hpp"

namespace Ui {
class RenderImage;
}

class RenderImage : public QMainWindow
{
    Q_OBJECT

public:
    explicit RenderImage( QWidget* parent = 0 );
    ~RenderImage();

public slots:
	void generateData();
	void distributeData();
	void paintData();

signals:
	void startGenerateData( datagen::DataGenerator * );
	void startDoStepData( datagen::DataGenerator * );
	
private:
    utils::auto_ptr<Ui::RenderImage> ui;
	utils::auto_ptr<WorkerThread> worker;
	utils::auto_ptr<datagen::DataGenerator> generator;
};

#endif // RENDERIMAGE_H
