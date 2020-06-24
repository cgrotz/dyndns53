# DynDns53
[![license](https://img.shields.io/github/license/cgrotz/dyndns53.svg)](https://github.com/cgrotz/dyndns53/blob/master/LICENSE)
[![release](https://img.shields.io/github/release/cgrotz/dyndns53.svg)](https://github.com/cgrotz/dyndns53/releases/latest)
![Continuous integration](https://github.com/cgrotz/dyndns53/workflows/Continuous%20integration/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/cgrotz/dyndns53/badge)](https://www.codefactor.io/repository/github/cgrotz/dyndns53)

A simple tool, to update a DNS entry in Route53 based on your current WAN IP address

## Usage
Simply call the executable dyndns53 and provide as first parameter the ID of your hosted zone in Route53 second parameter is the domain you want to use with a trailing point for example dyn.mydomain.org.