# led_publisher_node.py
import rclpy
from rclpy.node import Node
from std_msgs.msg import String

VALID_LEDS = {'red', 'green'}
VALID_MODES = {'on', 'off', 'blink'}

class LedControlPublisher(Node):
    def __init__(self):
        super().__init__('led_control_publisher')
        self.publisher = self.create_publisher(String, '/led_control', 10)

    def send_command(self, cmd):
        if not self.validate_command(cmd):
            self.get_logger().warn(f'Invalid LED command: "{cmd}" (Format: <led>:<mode>)')
            return

        msg = String()
        msg.data = cmd
        self.publisher.publish(msg)
        self.get_logger().info(f'Sent LED command: {cmd}')

    def validate_command(self, cmd):
        if ':' not in cmd:
            return False

        led, mode = map(str.strip, cmd.lower().split(':', 1))
        return led in VALID_LEDS and mode in VALID_MODES

def main():
    rclpy.init()
    node = LedControlPublisher()

    try:
        while True:
            cmd = input("Enter LED command (e.g. red:on, green:blink): ").strip()
            node.send_command(cmd)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()