use {
    crate::makepad_widgets::{ makepad_draw::*}
};

// the "IconButton" on the *left* hand side of the below is the name we will refer to the
// widget in the app's live_design block
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
   
    IconButton = <View> {
        align:{x:0.5,y:0.5}
        flow: Down,
        instance hover: 0.0
        instance pressed: 0.0
        image = <Image>{          
            width: 96,
            height: 96,
            source: dep("crate://self/resources/Icon1.png")
        }

        button = <Button> {
            width: Fill,
            text: "yes"
            draw_text: {
                text_style: {font_size: 15},
                //color: #f00
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.,0.,0.,0.)
                }
            }
        }       
    }
}
