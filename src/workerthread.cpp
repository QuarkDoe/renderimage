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

#include "workerthread.h"

void WorkerThread::run(){
	qDebug() << "WorkerThread::run";

	if( generator != NULL ){
		switch( work ){
			case DoGenerate:
				qDebug() << "generator->generate()";
				generator->generate();
				break;
			case DoDistribute:
				qDebug() << "generator->distribute()";
				generator->distribute();
				break;
		}
		generator = NULL;
	}
	qDebug() << "end WorkerThread::run";
	emit workFinished();
}

void WorkerThread::startGenerate(datagen::DataGenerator* g){
	qDebug() << "WorkerThread::startWork";
	if(isRunning()) wait();
	generator = g;
	work = DoGenerate;
	start(LowPriority);
}

void WorkerThread::startDistribute(datagen::DataGenerator* g){
	qDebug() << "WorkerThread::startWork";
	if(isRunning()) wait();
	generator = g;
	work = DoDistribute;
	start(LowPriority);
}
