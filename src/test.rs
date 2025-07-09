// test.rs
// Simple tests for the Knodiq Engine
//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sample_type() {
        let sample: Sample = 1.5;
        assert_eq!(sample, 1.5);

        let negative_sample: Sample = -0.8;
        assert_eq!(negative_sample, -0.8);
    }

    #[test]
    fn test_audio_buffer_basic() {
        let mut buffer = AudioBuffer::new();
        assert_eq!(buffer.len(), 0);

        // Add a channel with some samples
        buffer.push(vec![1.0, 2.0, 3.0]);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], vec![1.0, 2.0, 3.0]);

        // Add another channel
        buffer.push(vec![4.0, 5.0, 6.0]);
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer[1], vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_value_float() {
        let value = Value::Float(42.0);

        match value {
            Value::Float(f) => assert_eq!(f, 42.0),
            _ => panic!("Expected Value::Float"),
        }
    }

    #[test]
    fn test_value_array() {
        let values = vec![Value::Float(1.0), Value::Float(2.0)];
        let array_value = Value::Array(values);

        match array_value {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 2);
                match &arr[0] {
                    Value::Float(f) => assert_eq!(*f, 1.0),
                    _ => panic!("Expected first element to be Float"),
                }
            }
            _ => panic!("Expected Value::Array"),
        }
    }

    #[test]
    fn test_value_apply_fn() {
        let value = Value::Float(5.0);
        let result = value.apply_fn(|x| x * 2.0).unwrap();

        match result {
            Value::Float(f) => assert_eq!(f, 10.0),
            _ => panic!("Expected Value::Float"),
        }
    }

    #[test]
    fn test_value_apply_op() {
        let a = Value::Array(vec![Value::Array(vec![
            Value::Float(1.0),
            Value::Float(2.0),
        ])]);
        let b = Value::Array(vec![
            Value::Array(vec![Value::Float(3.0), Value::Float(4.0)]),
            Value::Array(vec![Value::Float(5.0), Value::Float(6.0)]),
            Value::Array(vec![Value::Float(7.0), Value::Float(8.0)]),
        ]);
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Float(4.0), Value::Float(6.0)]),
            Value::Array(vec![Value::Float(6.0), Value::Float(8.0)]),
            Value::Array(vec![Value::Float(8.0), Value::Float(10.0)]),
        ]);

        test_apply_op(a, b, expected);

        let a = Value::Array(vec![Value::Float(2.0)]);
        let b = Value::Array(vec![
            Value::Array(vec![
                Value::Float(3.0),
                Value::Float(4.0),
                Value::Float(5.0),
            ]),
            Value::Array(vec![
                Value::Float(5.0),
                Value::Float(6.0),
                Value::Float(7.0),
            ]),
        ]);
        let expected = Value::Array(vec![
            Value::Array(vec![
                Value::Float(5.0),
                Value::Float(6.0),
                Value::Float(7.0),
            ]),
            Value::Array(vec![
                Value::Float(7.0),
                Value::Float(8.0),
                Value::Float(9.0),
            ]),
        ]);

        test_apply_op(a, b, expected);

        let a = Value::Array(vec![
            Value::Array(vec![Value::Float(1.0)]),
            Value::Array(vec![Value::Float(2.0)]),
        ]);
        let b = Value::Array(vec![
            Value::Array(vec![
                Value::Float(1.0),
                Value::Float(2.0),
                Value::Float(3.0),
            ]),
            Value::Array(vec![
                Value::Float(1.0),
                Value::Float(2.0),
                Value::Float(5.0),
            ]),
        ]);
        let expected = Value::Array(vec![
            Value::Array(vec![
                Value::Float(2.0),
                Value::Float(3.0),
                Value::Float(4.0),
            ]),
            Value::Array(vec![
                Value::Float(3.0),
                Value::Float(4.0),
                Value::Float(7.0),
            ]),
        ]);

        test_apply_op(a, b, expected);
    }

    fn test_apply_op(a: Value, b: Value, expected: Value) {
        let result = Value::apply_op(&[&a, &b], |s| s[0] + s[1]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_node_id() {
        let id1 = NodeId::new_v4();
        let id2 = NodeId::new_v4();

        // IDs should be different
        assert_ne!(id1, id2);

        // Should be valid UUID format (36 characters with hyphens)
        assert_eq!(id1.to_string().len(), 36);
        assert_eq!(id2.to_string().len(), 36);
    }

    #[test]
    fn test_buffer_to_value_conversion() {
        let mut buffer = AudioBuffer::new();
        buffer.push(vec![1.0, 2.0]);
        buffer.push(vec![3.0, 4.0]);

        let value = Value::from_buffer(buffer.clone());
        let recovered_buffer = value.as_buffer().unwrap();

        assert_eq!(buffer.len(), recovered_buffer.len());
        assert_eq!(buffer[0], recovered_buffer[0]);
        assert_eq!(buffer[1], recovered_buffer[1]);
    }

    #[test]
    fn test_connector_creation() {
        let from_id = NodeId::new_v4();
        let to_id = NodeId::new_v4();

        let connector = Connector {
            from: from_id,
            from_param: "output".to_string(),
            to: to_id,
            to_param: "input".to_string(),
        };

        assert_eq!(connector.from, from_id);
        assert_eq!(connector.to, to_id);
        assert_eq!(connector.from_param, "output");
        assert_eq!(connector.to_param, "input");
    }

    #[test]
    fn test_beats_type() {
        let beats: Beats = 4.0;
        assert_eq!(beats, 4.0);

        let half_beat: Beats = 0.5;
        assert_eq!(half_beat, 0.5);
    }
}
