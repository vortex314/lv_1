client.subscribe("dst/*") >> queue(10,demux.thread(),DROP_OLDEST) >> demux.input();
demux.output("dst/device/motor/speed") >> motor.speedTarget();
demux.output("dst/device/sys/led") >> queue(1,thread2,OVERWRITE) >> led.on();
client.connected() >> publisher_connected() ;
publisher.connected() >> last() >> led.blinkFast();
publisher.connected() >> client.subscription();
demux.output("xx") >> via([](){ return Some(in=="on");} ) >> led.on();

Some(publisher);

>> map(|x| { return Some(x=="true");}) >>
>> queue(!0) >> demux.input() 
timer(thread,100) >> map(|t| { return "Test-message"; }) >> uart.txd();

mqtt.connected() >> map(|b| { return b?1000:100 ; }) >> led.blinkInterval();
mqtt.connected() >> sink(|x| { info!(" Mqtt is {}connected.",x?"":"dis"); })