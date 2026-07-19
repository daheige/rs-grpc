// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var hello_pb = require('./hello_pb.js');
// var google_api_annotations_pb = require('./google/api/annotations_pb.js');

function serialize_Hello_HealthzReply(arg) {
  if (!(arg instanceof hello_pb.HealthzReply)) {
    throw new Error('Expected argument of type Hello.HealthzReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HealthzReply(buffer_arg) {
  return hello_pb.HealthzReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Hello_HealthzReq(arg) {
  if (!(arg instanceof hello_pb.HealthzReq)) {
    throw new Error('Expected argument of type Hello.HealthzReq');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HealthzReq(buffer_arg) {
  return hello_pb.HealthzReq.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Hello_HelloReply(arg) {
  if (!(arg instanceof hello_pb.HelloReply)) {
    throw new Error('Expected argument of type Hello.HelloReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HelloReply(buffer_arg) {
  return hello_pb.HelloReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Hello_HelloReq(arg) {
  if (!(arg instanceof hello_pb.HelloReq)) {
    throw new Error('Expected argument of type Hello.HelloReq');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HelloReq(buffer_arg) {
  return hello_pb.HelloReq.deserializeBinary(new Uint8Array(buffer_arg));
}


// Greeter service 定义开放调用的服务
var GreeterService = exports.GreeterService = {
  healthz: {
    path: '/Hello.Greeter/Healthz',
    requestStream: false,
    responseStream: false,
    requestType: hello_pb.HealthzReq,
    responseType: hello_pb.HealthzReply,
    requestSerialize: serialize_Hello_HealthzReq,
    requestDeserialize: deserialize_Hello_HealthzReq,
    responseSerialize: serialize_Hello_HealthzReply,
    responseDeserialize: deserialize_Hello_HealthzReply,
  },
  sayHello: {
    path: '/Hello.Greeter/SayHello',
    requestStream: false,
    responseStream: false,
    requestType: hello_pb.HelloReq,
    responseType: hello_pb.HelloReply,
    requestSerialize: serialize_Hello_HelloReq,
    requestDeserialize: deserialize_Hello_HelloReq,
    responseSerialize: serialize_Hello_HelloReply,
    responseDeserialize: deserialize_Hello_HelloReply,
  },
};

exports.GreeterClient = grpc.makeGenericClientConstructor(GreeterService, 'Greeter');
