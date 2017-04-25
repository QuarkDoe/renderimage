#include "renderimage.h"
#include "ui_renderimage.h"
#include <QtDebug>
#include <QPainter>

RenderImage::RenderImage(QWidget *parent) : QMainWindow(parent) {
	
	generator = new datagen::DataGenerator();
	
	ui = new Ui::RenderImage();
	ui->setupUi(this);

	worker = new WorkerThread();
	QObject::connect(
		this,
		SIGNAL(startGenerateData(datagen::DataGenerator *)),
		worker.get(),
		SLOT(startGenerate(datagen::DataGenerator *))
	);
	
	QObject::connect(
		this,
		SIGNAL(startDoStepData(datagen::DataGenerator *)),
		worker.get(),
		SLOT(startDistribute(datagen::DataGenerator *))
	);
	
	QObject::connect(
		worker.get(),
		SIGNAL(workFinished()),
		this,
		SLOT(paintData())
	);
	
	qDebug() << "RenderImage::RenderImage";
}

RenderImage::~RenderImage()
{
	qDebug() << "RenderImage::~RenderImage";
	if( !worker.empty() && worker->isRunning() ) worker->wait();
}

void RenderImage::generateData(){
	qDebug() << "RenderImage::renderData";
	ui->renderButton->setEnabled(false);
	ui->stepButton->setEnabled(false);
	generator->set_size( ui->widthSpinBox->value(), ui->heightSpinBox->value() );
	generator->set_item_number( ui->numberSpinBox->value() );
	emit startGenerateData( generator );
}

void RenderImage::distributeData(){
	qDebug() << "RenderImage::doStep";
	ui->renderButton->setEnabled(false);
	ui->stepButton->setEnabled(false);
	emit startDoStepData( generator );
}

void RenderImage::paintData(){
	qDebug() << "RenderImage::paintData";

	if(generator.empty()){
		qDebug() << "RenderImage::paintData: generator is empty";
	}else{
		float width = 0, height = 0;
		generator->get_size( &width, &height );
		qDebug() << "RenderImage::paintData: gsize{ width:" << width << " height:" << height << " }";			
		
		utils::auto_ptr<QImage> image = new QImage( width, height, QImage::Format_RGB32 );
		image->fill( Qt::white );
		
		{
			utils::auto_ptr<QPainter> painter = new QPainter();
			painter->begin(image);
			QRectF r;
			QPointF p1, p2;
			datagen::gen_point_f32 center;
			QString str;
			QTextStream sstream( &str );
			
			for( size_t idx = 0; idx < generator->len(); idx++ ){
				const datagen::gen_rect_f32* rect = generator->get_rect( idx );
				const datagen::gen_vector_f32* vect = generator->get_vect( idx );
				datagen::gen_type type = generator->get_type( idx );
				

				
				r.setCoords( rect->p1.x, rect->p1.y, rect->p2.x, rect->p2.y );
// 				qDebug() << "rect:" << r << "type:" << type << endl;

				painter->setPen(
					QPen(
						type == datagen::Common ? Qt::green : Qt::red,
						1,
						Qt::SolidLine,
						Qt::RoundCap,
						Qt::MiterJoin
					)
				);
				painter->drawRect( r );

				str.clear();
				sstream << idx;
				painter->setPen( QPen( Qt::darkGray, 1, Qt::SolidLine, Qt::RoundCap, Qt::MiterJoin ) );
				painter->drawText( r, Qt::AlignCenter, str );
				
				generator->get_center( idx, &center );
				p1.setX( center.x );
				p1.setY( center.y );
				p2.setX( center.x + vect->dx );
				p2.setY( center.y + vect->dy );
				painter->setPen( QPen( Qt::blue, 1, Qt::SolidLine, Qt::RoundCap, Qt::MiterJoin ) );
				painter->drawLine( p1, p2 );
			}
			painter->end();
		}
		
		ui->paintedLabel->setPixmap( QPixmap::fromImage( *image ) );
	}
	
	ui->renderButton->setEnabled(true);
	ui->stepButton->setEnabled(true);
}
