#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include <rcl/rcl.h>
#include <rclc/executor.h>
#include <rclc/rclc.h>
#include <std_msgs/msg/string.h>
#include <string.h>

// Global variables
rcl_publisher_t publisher;
rcl_subscription_t subscriber;
std_msgs__msg__String msg_out;
rclc_executor_t executor;
rcl_timer_t timer;

int ping_count = 0;

void pong_callback(const void *msgin) {
  const std_msgs__msg__String *msg = (const std_msgs__msg__String *)msgin;
  // For this minimal version, we do nothing with the pong
  (void)msg;
}

void timer_callback(rcl_timer_t *timer, int64_t last_call_time) {
  RCLC_UNUSED(last_call_time);
  if (timer == NULL)
    return;

  static char ping_buffer[50];
  snprintf(ping_buffer, sizeof(ping_buffer), "ping 🏓 #%d", ping_count++);

  msg_out.data.data = ping_buffer;
  msg_out.data.size = strlen(ping_buffer);
  msg_out.data.capacity = sizeof(ping_buffer);

  rcl_publish(&publisher, &msg_out, NULL);
}

void appMain(void *arg) {
  rcl_allocator_t allocator = rcl_get_default_allocator();
  rclc_support_t support;

  // Initialize support structure
  rclc_support_init(&support, 0, NULL, &allocator);

  // Create the ROS node
  rcl_node_t node;
  rclc_node_init_default(&node, "wireless_table_tennis_node", "", &support);

  // Initialize publisher on /ping
  rclc_publisher_init_default(
      &publisher, &node, ROSIDL_GET_MSG_TYPE_SUPPORT(std_msgs, msg, String),
      "ping");

  // Initialize subscriber on /pong
  rclc_subscription_init_default(
      &subscriber, &node, ROSIDL_GET_MSG_TYPE_SUPPORT(std_msgs, msg, String),
      "pong");

  // Set up timer to publish every 1 second
  rclc_timer_init_default(&timer, &support, RCL_MS_TO_NS(1000), timer_callback);

  // Set up executor to manage callbacks
  executor = rclc_executor_get_zero_initialized_executor();
  rclc_executor_init(&executor, &support.context, 2, &allocator);
  rclc_executor_add_timer(&executor, &timer);
  rclc_executor_add_subscription(&executor, &subscriber, &msg_out,
                                 pong_callback, ON_NEW_DATA);

  // Main loop
  while (true) {
    rclc_executor_spin_some(&executor, RCL_MS_TO_NS(100));
    vTaskDelay(pdMS_TO_TICKS(100)); // 100 ms delay
  }
}
