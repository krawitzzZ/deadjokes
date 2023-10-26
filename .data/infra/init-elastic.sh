#!/bin/bash

curl --retry 10 --retry-delay 3 --retry-connrefused -XPUT 'http://localhost:9200/deadjokes_logs_index' -H 'Content-Type: application/json' -d'
{
    "settings": {
        "number_of_shards": 1,
        "number_of_replicas": 0
    }
}
'
