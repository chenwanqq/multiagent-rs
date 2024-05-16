# multiagent-rs
a rust multi llm agent project

## 思路
参考metaGPT

对于单个智能体，实现：REACT, BY_ORDER, PLAN_AND_ACT三种模式

智能体的通信，使用watch（订阅）某个action实现

对llm的调用，目前使用的是erniebot-rs，等待llm-chain（1）将我的ernie后端合并；（2）executor变为object-safe后再采用

## 组成部分
* action
* role
* environment


