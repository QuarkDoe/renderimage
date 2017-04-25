/*
 * Copyright 2016 <copyright holder> <email>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef WORKERTHREAD_H
#define WORKERTHREAD_H

#include <QThread>
#include <QtDebug>

#include "datagenerator.h"
#include "utils/container.hpp"

typedef enum work_type {
	DoGenerate,
	DoDistribute,
} work_type;

class WorkerThread :  public QThread {
	Q_OBJECT

protected:
	virtual void run();

signals:
	void workFinished();   

public slots:
	void startGenerate(datagen::DataGenerator *);
	void startDistribute(datagen::DataGenerator *);

private:
// 	utils::auto_ptr<datagen::DataGenerator> generator;
	work_type work;
	datagen::DataGenerator* generator;
	void runThread();
};

#endif // WORKERTHREAD_H
