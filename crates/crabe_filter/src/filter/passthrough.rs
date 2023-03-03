use nalgebra::Point2;
use ringbuffer::RingBufferRead;
use crabe_framework::data::world::{Ball, Robot, World};
use crate::data::{FilterData, TrackedBall, TrackedRobot};
use crate::data::camera::CamBall;
use crate::filter::Filter;

fn robot_passthrough<'a, T: 'a + Default>(
    robots: impl Iterator<Item=(&'a u32, &'a mut TrackedRobot<T>)>
) {
    robots.for_each(|(_id, r)| {
        let last_packet = r.packets.drain().last();
        if let Some(packet) = last_packet {
            r.data = Robot {
                id: packet.id,
                position: packet.position,
                orientation: packet.orientation,
                has_ball: false,
                robot_info: T::default(),
            }
        }
    } )
}

fn ball_passthrough(
    ball: &mut TrackedBall
) {
    let last_packet = ball.packets.drain().last();
    if let Some(packet) = last_packet {
        ball.data = Ball {
            position: packet.position
        }
    }
}


pub struct PassthroughFilter;

impl Filter for PassthroughFilter {
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        robot_passthrough(filter_data.allies.iter_mut());
        robot_passthrough(filter_data.enemies.iter_mut());
        ball_passthrough(&mut filter_data.ball);
    }
}