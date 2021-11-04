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
*AppointmentsApi* | [**get_appointments**](docs/AppointmentsApi.md#get_appointments) | **GET** /appointments | List all appointments.
*AppointmentsApi* | [**get_available_dates**](docs/AppointmentsApi.md#get_available_dates) | **GET** /scheduling/available-dates | Fetch available days for a user or group
*AppointmentsApi* | [**get_available_timeslots**](docs/AppointmentsApi.md#get_available_timeslots) | **GET** /scheduling/available-timeslots | Fetch available timeslots for a user or group
*AppointmentsApi* | [**get_unconfirmed_appointments**](docs/AppointmentsApi.md#get_unconfirmed_appointments) | **GET** /unconfirmed-appointments | List all unconfirmed appointments.
*AppointmentsApi* | [**get_unconfirmed_appointments_id**](docs/AppointmentsApi.md#get_unconfirmed_appointments_id) | **GET** /unconfirmed-appointments/{unconfirmed_appointment_id} | Retrieve an unconfirmed appointment.
*AppointmentsApi* | [**put_appointments_appointment_id_cancel**](docs/AppointmentsApi.md#put_appointments_appointment_id_cancel) | **PUT** /appointments/{appointment_id}/cancel | Cancel an appointment.
*AppointmentsApi* | [**put_appointments_appointment_id_reschedule**](docs/AppointmentsApi.md#put_appointments_appointment_id_reschedule) | **PUT** /appointments/{appointment_id}/reschedule | Reschedule an appointment.
*ListingsApi* | [**get_listings**](docs/ListingsApi.md#get_listings) | **GET** /listings | List all listings.
*ListingsApi* | [**get_listings_id**](docs/ListingsApi.md#get_listings_id) | **GET** /listings/{listing_id} | Retrieve a listing.
*OrdersApi* | [**get_orders**](docs/OrdersApi.md#get_orders) | **GET** /orders | List all orders.
*OrdersApi* | [**get_orders_id**](docs/OrdersApi.md#get_orders_id) | **GET** /orders/{order_id} | Retrieve an order.
*OrdersApi* | [**get_products**](docs/OrdersApi.md#get_products) | **GET** /products | List all products.
*OrdersApi* | [**post_orders**](docs/OrdersApi.md#post_orders) | **POST** /orders | Create an order.
*VendorsApi* | [**get_vendors**](docs/VendorsApi.md#get_vendors) | **GET** /vendors | List all vendors.
*VendorsApi* | [**get_vendors_id**](docs/VendorsApi.md#get_vendors_id) | **GET** /vendors/{vendor_id} | Retrieve a vendor.


## Documentation For Models

 - [Address](docs/Address.md)
 - [ApiError403](docs/ApiError403.md)
 - [ApiError404](docs/ApiError404.md)
 - [ApiError409](docs/ApiError409.md)
 - [ApiError500](docs/ApiError500.md)
 - [ApiFail422](docs/ApiFail422.md)
 - [Appointment](docs/Appointment.md)
 - [AppointmentCancelPutPayload](docs/AppointmentCancelPutPayload.md)
 - [AppointmentCollection](docs/AppointmentCollection.md)
 - [AppointmentReschedulePutPayload](docs/AppointmentReschedulePutPayload.md)
 - [AppointmentResource](docs/AppointmentResource.md)
 - [CalendarDay](docs/CalendarDay.md)
 - [CalendarDayCollection](docs/CalendarDayCollection.md)
 - [FloorPlan](docs/FloorPlan.md)
 - [Group](docs/Group.md)
 - [GroupCollection](docs/GroupCollection.md)
 - [GroupResource](docs/GroupResource.md)
 - [Image](docs/Image.md)
 - [InteractiveContent](docs/InteractiveContent.md)
 - [Listing](docs/Listing.md)
 - [ListingBuilding](docs/ListingBuilding.md)
 - [ListingCollection](docs/ListingCollection.md)
 - [ListingLot](docs/ListingLot.md)
 - [ListingPrice](docs/ListingPrice.md)
 - [ListingResource](docs/ListingResource.md)
 - [Order](docs/Order.md)
 - [OrderCollection](docs/OrderCollection.md)
 - [OrderForm](docs/OrderForm.md)
 - [OrderItem](docs/OrderItem.md)
 - [OrderPostPayload](docs/OrderPostPayload.md)
 - [OrderResource](docs/OrderResource.md)
 - [PaginationLinks](docs/PaginationLinks.md)
 - [PaginationMeta](docs/PaginationMeta.md)
 - [Product](docs/Product.md)
 - [ProductCategory](docs/ProductCategory.md)
 - [ProductCollection](docs/ProductCollection.md)
 - [ProductVariant](docs/ProductVariant.md)
 - [PropertyWebsite](docs/PropertyWebsite.md)
 - [SocialProfiles](docs/SocialProfiles.md)
 - [Timeslot](docs/Timeslot.md)
 - [TimeslotCollection](docs/TimeslotCollection.md)
 - [UnconfirmedAppointment](docs/UnconfirmedAppointment.md)
 - [UnconfirmedAppointmentCollection](docs/UnconfirmedAppointmentCollection.md)
 - [UnconfirmedAppointmentResource](docs/UnconfirmedAppointmentResource.md)
 - [User](docs/User.md)
 - [Video](docs/Video.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

jarrod@aryeo.com

