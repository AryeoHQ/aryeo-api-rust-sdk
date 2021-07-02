# Aryeo SDK

## Introduction

This is an auto-generated client SDK for interfacing with the Aryeo API. We support a variety of languages and frameworks that are a great starting point for experimenting with the API. If there is an additional language or framework that you want to see supported, then please reach out and make a contribution!

<p align="center"> <a href="https://github.com/AryeoHQ/aryeo-api-dart-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/dart.svg" alt="Dart" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-go-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/go.svg" alt="Go" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-js-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/js.svg" alt="Node JS" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-php-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/php.svg" alt="PHP" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-ruby-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/ruby.svg" alt="Ruby" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-rust-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/rust.svg" alt="Rust" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> <a href="https://github.com/AryeoHQ/aryeo-api-swift-sdk"><img src="https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/swift.svg" alt="Swift" width="44" style="padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;"/></a> </p>

## Authentication

To start using the Aryeo API, you will need to generate an API key from your group's developer settings. Then, make sure to provide your API key as a bearer token. Requests made without an API key will result in a `401 Unauthorized` error.

Example: `Authorization: Bearer {API_KEY}`

## Installation

Add the following block to `Cargo.toml`:

```
[dependencies]
aryeo = { git = "https://github.com/AryeoHQ/aryeo-api-rust-sdk", branch = "master" }
```

## Getting Started

```rust
use aryeo::apis::{configuration::Configuration, listings_api as aryeo_api};

#[tokio::main]
async fn main() {
    let mut config = Configuration::default();    
    config.bearer_access_token = Some(String::from("API_KEY"));
    
    let result = aryeo_api::get_listings_id(
        &config, 
        "UUID"
    )
    .await;

    let result = match result {
        Ok(result) => result,
        Err(error) => panic!("Something went wrong, {:?}", error),
    };
    
    println!("{:?}", result);
}
```

## Documentation for API Endpoints

All URIs are relative to *https://api.aryeo.com/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ListingsApi* | [**get_listings**](docs/ListingsApi.md#get_listings) | **GET** /listings | Get the listings available to a group.
*ListingsApi* | [**get_listings_id**](docs/ListingsApi.md#get_listings_id) | **GET** /listings/{uuid} | Get information about a listing.
*OrdersApi* | [**get_orders**](docs/OrdersApi.md#get_orders) | **GET** /orders | Get orders available to a group.
*OrdersApi* | [**post_orders**](docs/OrdersApi.md#post_orders) | **POST** /orders | Create an order.
*VendorsApi* | [**get_vendors**](docs/VendorsApi.md#get_vendors) | **GET** /vendors | Get vendors available to a group.
*VendorsApi* | [**get_vendors_search**](docs/VendorsApi.md#get_vendors_search) | **GET** /vendors/search | Get vendors that can be added to the group's vendor list.


## Documentation For Models

 - [ApiError](docs/ApiError.md)
 - [Currency](docs/Currency.md)
 - [FloorPlan](docs/FloorPlan.md)
 - [Group](docs/Group.md)
 - [GroupAgentProperties](docs/GroupAgentProperties.md)
 - [GroupCollection](docs/GroupCollection.md)
 - [Image](docs/Image.md)
 - [InteractiveContent](docs/InteractiveContent.md)
 - [Listing](docs/Listing.md)
 - [ListingResource](docs/ListingResource.md)
 - [Order](docs/Order.md)
 - [OrderCollection](docs/OrderCollection.md)
 - [OrderForm](docs/OrderForm.md)
 - [OrderPostPayload](docs/OrderPostPayload.md)
 - [OrderResource](docs/OrderResource.md)
 - [PaginationLinks](docs/PaginationLinks.md)
 - [PaginationMeta](docs/PaginationMeta.md)
 - [PartialAddress](docs/PartialAddress.md)
 - [PartialGroup](docs/PartialGroup.md)
 - [PartialListing](docs/PartialListing.md)
 - [PartialListingCollection](docs/PartialListingCollection.md)
 - [ProductItem](docs/ProductItem.md)
 - [PropertyDetails](docs/PropertyDetails.md)
 - [PropertyWebsites](docs/PropertyWebsites.md)
 - [SocialProfiles](docs/SocialProfiles.md)
 - [User](docs/User.md)
 - [Video](docs/Video.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

jarrod@aryeo.com

