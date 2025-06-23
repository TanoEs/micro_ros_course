import rclpy
from rclpy.node import Node
from std_msgs.msg import String


class LEDControlNode(Node):
    def __init__(self):
        super().__init__('led_control_node')
        self.publisher = self.create_publisher(String, 'led_control', 10)
        self.get_logger().info("LED control node is ready!")

        self.subscriber = self.create_subscription(
            String, 'led_status', self.status_callback, 10)
        self.latest_status = None
        self.get_logger().info('LED Controller initialized')

    def status_callback(self, msg):
        self.latest_status = msg.data
        self.get_logger().info(f'LED Status update: {msg.data}')

    def publish_cmd(self, cmd_str: str):
        msg = String()
        msg.data = cmd_str
        self.publisher.publish(msg)
        self.get_logger().info(f'Sent control: "{cmd_str}"')


def main(args=None):
    rclpy.init(args=args)
    node = LEDControlNode()
    try:
        while rclpy.ok():
            # let any incoming statuses arrive
            rclpy.spin_once(node, timeout_sec=0.1)
            print('\n=== LED Menu ===')
            print('1) status   2) on   3) off   q) quit')
            choice = input('Your choice: ').strip().lower()
            if choice in ('1', 'status'):
                s = node.latest_status or '<no status yet>'
                print(f'>> Latest LED status: {s}')
            elif choice in ('2', 'on'):
                node.publish_cmd('on')
            elif choice in ('3', 'off'):
                node.publish_cmd('off')
            elif choice in ('q', 'quit'):
                break
            else:
                print('>> Invalid option')
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
