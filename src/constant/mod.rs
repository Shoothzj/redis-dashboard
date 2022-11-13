/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLUSTER_ENABLE: bool = {
        match std::env::var("CLUSTER_ENABLE") {
            Ok(enable) => {
                if enable == "true" {
                    true
                } else {
                    false
                }
            }
            Err(..) => {
                false
            }
        }
    };
    pub static ref REDIS_URL: String = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    pub static ref REDIS_CLUSTER_URL: String = std::env::var("REDIS_CLUSTER_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
}
